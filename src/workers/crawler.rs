use std::sync::{Arc, RwLock};
use std::time::Duration;

use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use spider::configuration::{WaitForDelay, WaitForSelector};
use spider::tokio;
use spider::website::Website;
use spider::{
    configuration::WaitForIdleNetwork, features::chrome_common::RequestInterceptConfiguration,
};
use spider_transformations::transformation::content;
use tantivy::{doc, Index, Opstamp, TantivyDocument};
use tokio::io::AsyncWriteExt;

use crate::app::{self, tantivy_index};

pub struct CrawlerWorker {
    pub ctx: AppContext,
    pub index: Arc<Index>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CrawlerWorkerArgs {
    pub url: String,
}

#[async_trait]
impl BackgroundWorker<CrawlerWorkerArgs> for CrawlerWorker {
    fn build(ctx: &AppContext) -> Self {
        Self {
            ctx: ctx.clone(),
            index: app::tantivy_index.clone(),
        }
    }
    async fn perform(&self, args: CrawlerWorkerArgs) -> Result<()> {
        let mut stdout = tokio::io::stdout();
        let mut interception = RequestInterceptConfiguration::new(true);

        interception.block_javascript = true;

        let mut website: Website = Website::new(args.url.as_str())
            .with_limit(10000)
            .with_chrome_intercept(interception)
            .with_wait_for_idle_network(Some(WaitForIdleNetwork::new(Some(Duration::from_millis(
                500,
            )))))
            .with_wait_for_idle_dom(Some(WaitForSelector::new(
                Some(Duration::from_millis(100)),
                "body".into(),
            )))
            .with_block_assets(true)
            // .with_wait_for_delay(Some(WaitForDelay::new(Some(Duration::from_millis(10000)))))
            .with_stealth(true)
            .with_return_page_links(true)
            .with_fingerprint(true)
            // .with_proxies(Some(vec!["http://localhost:8888".into()]))
            // .with_chrome_connection(Some("http://127.0.0.1:9222/json/version".into()))
            .build()
            .unwrap();

        let mut rx2 = website.subscribe(16).unwrap();

        let start = spider::tokio::time::Instant::now();
        

        let index_writer = Arc::new(RwLock::new(tantivy_index.writer(50_000_000).unwrap()));

        // let index_writer = tantivy_writer.clone();

        let title = tantivy_index.schema().get_field("title").unwrap();
        let url = tantivy_index.schema().get_field("title").unwrap();
        let body = tantivy_index.schema().get_field("title").unwrap();

        let index_writer_lock = index_writer.clone();

        let (links, _) = tokio::join!(
            async move {
                website.crawl().await;
                website.unsubscribe();
                website.get_links()
            },
            async move {
                while let Ok(page) = rx2.recv().await {
                    let conf = content::TransformConfig::default();
                    let content = content::transform_content(&page, &conf, &None, &None, &None);

                    index_writer_lock.read().unwrap().add_document(doc!(
                        title => "",
                        url => page.get_url().to_string(),
                        body => content,
                    ));

                    let _ = stdout
                        .write_all(
                            format!(
                                "- {} -- Bytes transferred {:?} -- HTML Size {:?} -- Links: {:?}\n",
                                page.get_url(),
                                page.bytes_transferred.unwrap_or_default(),
                                page.get_html_bytes_u8().len(),
                                match page.page_links {
                                    Some(ref l) => l.len(),
                                    _ => 0,
                                }
                            )
                            .as_bytes(),
                        )
                        .await;
                }
            }
        );

        let duration = start.elapsed();


        let opstamp: Opstamp = {
            let mut index_writer_commit = index_writer.write().unwrap();
            index_writer_commit.commit().unwrap()
        };
        println!("committed with opstamp {opstamp}");

        println!(
            "Time elapsed in website.crawl({}) is: {:?} for total pages: {:?}",
            args.url,
            duration,
            links.len()
        );

        Ok(())
    }
}