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
use spider_transformations::transformation::content::{self, ReturnFormat};
use tantivy::{doc, Index, Opstamp, TantivyDocument, Term};
use tracing::info;
use ActiveValue::NotSet;

use crate::app::{self, tantivy_index, tantivy_writer};

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
        info!("Crawling {}", args.url);

        // Create website_db or update existing
        // let mut website_model = websites::Entity::find()
        //     .filter(websites::Column::Domain.eq(args.url.clone()))
        //     .one(&self.ctx.db)
        //     .await?;

        // if (website_model.is_none()) {
        //     let mut website_model = websites::ActiveModel {
        //         id: NotSet, // primary key is NotSet
        //         domain: Set(Some(args.url.clone())),
        //         ..Default::default() // all other attributes are `NotSet`
        //     };
        //     let website_model = website_model.insert(&self.ctx.db).await?;
        // }

        let mut website: Website = Website::new(args.url.as_str())
            .with_limit(10000)
            .build()
            .unwrap();
            // .with_chrome_intercept(interception)
            // .with_wait_for_idle_network(Some(WaitForIdleNetwork::new(Some(Duration::from_millis(
            //     500,
            // )))))
            // .with_wait_for_idle_dom(Some(WaitForSelector::new(
            //     Some(Duration::from_millis(100)),
            //     "body".into(),
            // )))
            // .with_block_assets(true)
            // // .with_wait_for_delay(Some(WaitForDelay::new(Some(Duration::from_millis(10000)))))
            // .with_stealth(true)
            // .with_return_page_links(true)
            // .with_fingerprint(true)
            // // .with_proxies(Some(vec!["http://localhost:8888".into()]))
            // // .with_chrome_connection(Some("http://127.0.0.1:9222/json/version".into()))
            // .build()
            // .unwrap();

        let mut rx2 = website.subscribe(16).unwrap();

        let start = spider::tokio::time::Instant::now();

        // let index_writer = tantivy_writer.clone();

        let title = tantivy_index.schema().get_field("title").unwrap();
        let url = tantivy_index.schema().get_field("url").unwrap();
        let body = tantivy_index.schema().get_field("body").unwrap();

        let index_writer_lock = tantivy_writer.clone();

        let (links, _) = tokio::join!(
            async move {
                website.crawl().await;
                website.unsubscribe();
                website.get_links()
            },
            async move {
                while let Ok(page) = rx2.recv().await {
                    let mut conf = content::TransformConfig::default();
                    conf.return_format = ReturnFormat::Html2Text;
                    let content = content::transform_content(&page, &conf, &None, &None, &None);

                    let doc_url = Term::from_field_text(url, page.get_url());

                    index_writer_lock
                        .read()
                        .unwrap()
                        .delete_term(doc_url.clone());
                    
                    index_writer_lock
                        .read()
                        .unwrap()
                        .add_document(doc!(
                            title => "",
                            url => page.get_url().to_string(),
                            body => content.clone(),
                        ))
                        .unwrap();

                    info!(
                        "- {} -- Bytes transferred {:?} -- HTML Size {:?} -- Links: {:?}",
                        page.get_url(),
                        page.bytes_transferred.unwrap_or_default(),
                        page.get_html_bytes_u8().len(),
                        match page.page_links {
                            Some(ref l) => l.len(),
                            _ => 0,
                        }
                    )
                }
            }
        );

        let opstamp: Opstamp = {
            let mut index_writer_commit = tantivy_writer.write().unwrap();
            index_writer_commit.commit().unwrap()
        };

        let duration = start.elapsed();

        info!(
            "Time elapsed in website.crawl({}) is: {:?} for total pages: {:?}, index opstamp: {:?}",
            args.url,
            duration,
            links.len(),
            opstamp
        );

        Ok(())
    }
}
