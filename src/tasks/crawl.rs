use loco_rs::prelude::*;

use crate::workers::crawl_all::{CrawlAllWorker, CrawlAllWorkerArgs};

#[allow(clippy::module_name_repetitions)]
pub struct Crawl;
#[async_trait]
impl Task for Crawl {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "crawl".to_string(),
            detail: "Task for crawling websites".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        CrawlAllWorker::perform_later(app_context, CrawlAllWorkerArgs {}).await?;

        Ok(())
    }
}
