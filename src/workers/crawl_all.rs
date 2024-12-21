use sea_orm::QuerySelect;
use serde::{Deserialize, Serialize};
use loco_rs::prelude::*;

use tracing::info;

use crate::{models::_entities::websites, workers::crawler::{CrawlerWorker, CrawlerWorkerArgs}};

pub struct CrawlAllWorker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CrawlAllWorkerArgs {
}

#[async_trait]
impl BackgroundWorker<CrawlAllWorkerArgs> for CrawlAllWorker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }
    async fn perform(&self, _args: CrawlAllWorkerArgs) -> Result<()> {
        println!("=================CrawlAll=======================");

        let websites = websites::Entity::find()
            .limit(5)
            .all(&self.ctx.db)
            .await?;

        info!("Enqueuing {} websites to crawl", websites.len());

        for website in websites {
            CrawlerWorker::perform_later(&self.ctx, CrawlerWorkerArgs { url: website.domain }).await?;
        }

        Ok(())
    }
}
