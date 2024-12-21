use loco_rs::prelude::*;
use sea_orm::QuerySelect;
use serde::{Deserialize, Serialize};

use tracing::info;

use crate::{
    app::tantivy_writer, models::_entities::websites, workers::crawler::{CrawlerWorker, CrawlerWorkerArgs}
};

pub struct CrawlAllWorker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CrawlAllWorkerArgs {}

#[async_trait]
impl BackgroundWorker<CrawlAllWorkerArgs> for CrawlAllWorker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }
    async fn perform(&self, _args: CrawlAllWorkerArgs) -> Result<()> {
        println!("=================CrawlAll=======================");

        {
            info!("Removing index...");

            let mut index_writer = tantivy_writer.write().unwrap();
            index_writer.delete_all_documents().unwrap();
            index_writer.commit().unwrap();
        }

        let websites = websites::Entity::find().limit(10).all(&self.ctx.db).await?;

        info!("Enqueuing {} websites to crawl", websites.len());

        for website in websites {
            CrawlerWorker::perform_later(
                &self.ctx,
                CrawlerWorkerArgs {
                    url: website.domain,
                },
            )
            .await?;
        }

        Ok(())
    }
}
