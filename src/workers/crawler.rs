use std::sync::{Arc, RwLock};
use std::time::Duration;

use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use spider::configuration::{WaitForIdleNetwork, WaitForSelector};
use spider::website::Website;
use spider::{features::chrome_common::RequestInterceptConfiguration, tokio};
use spider_transformations::transformation::content::{self, ReturnFormat, TransformConfig};
use tantivy::{doc, Index, Opstamp, TantivyError, Term};
use tracing::info;
use ActiveValue::NotSet;

use crate::models::_entities::pages;
use crate::{
    app::{self, tantivy_index, tantivy_writer},
    models::_entities::websites,
};

fn update_index(
    index_writer_lock: &Arc<RwLock<tantivy::IndexWriter>>,
    url: String,
    content: String,
) -> Result<(), TantivyError> {
    let title_field = tantivy_index.schema().get_field("title").unwrap();
    let url_field = tantivy_index.schema().get_field("url").unwrap();
    let body_field = tantivy_index.schema().get_field("body").unwrap();

    let index_writer = index_writer_lock.read()?;

    let doc_url = Term::from_field_text(url_field, url.as_str());
    index_writer.delete_term(doc_url.clone());

    index_writer.add_document(doc!(
        title_field => "",
        url_field => url,
        body_field => content,
    ))?;

    Ok(())
}

async fn create_or_get_website(db: &DatabaseConnection, url: String) -> Result<websites::Model> {
    let website_model = websites::Entity::find()
        .filter(websites::Column::Domain.eq(url.clone()))
        .one(db)
        .await?;

    match website_model {
        None => {
            let website_model = websites::ActiveModel {
                id: NotSet, // primary key is NotSet
                domain: Set(url),
                ..Default::default() // all other attributes are `NotSet`
            };
            website_model.insert(db).await.map_err(Into::into)
        }
        Some(website_model) => Ok(website_model),
    }
}

async fn create_or_update_page(
    db: &DatabaseConnection,
    url: String,
    website_id: i32,
    content: String,
) -> Result<pages::Model> {
    let page_model = pages::Entity::find()
        .filter(pages::Column::Url.eq(url.clone()))
        .one(db)
        .await?;

    if let Some(existing_page) = page_model {
        // Update existing page
        let mut page_model: pages::ActiveModel = existing_page.into();
        page_model.body = Set(content);
        page_model.update(db).await
    } else {
        // Create new page
        let page_model = pages::ActiveModel {
            id: NotSet,
            url: Set(url),
            website_id: Set(website_id),
            body: Set(content),
            ..Default::default()
        };
        page_model.insert(db).await
    }
    .map_err(Into::into)
}

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

        let website_model = create_or_get_website(&self.ctx.db, args.url.clone()).await?;

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

        let index_writer_lock = tantivy_writer.clone();

        let (links, _) = tokio::join!(
            async move {
                website.crawl().await;
                website.unsubscribe();
                website.get_links()
            },
            async move {
                while let Ok(page) = rx2.recv().await {
                    let conf = TransformConfig {
                        return_format: ReturnFormat::Html2Text,
                        ..Default::default()
                    };
                    let content = content::transform_content(&page, &conf, &None, &None, &None);
                    let url = page.get_url().to_string();

                    update_index(&index_writer_lock, url.clone(), content.clone()).unwrap_or_else(
                        |e| {
                            eprintln!("Error updating index: {:?}", e);
                        },
                    );

                    match create_or_update_page(
                        &self.ctx.db,
                        url.clone(),
                        website_model.id,
                        content.clone(),
                    )
                    .await
                    {
                        Ok(_) => (),
                        Err(e) => eprintln!("Error creating/updating page: {:?}", e),
                    }

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
