//! This task implements data seeding functionality for initializing new
//! development/demo environments.
//!
//! # Example
//!
//! Run the task with the following command:
//! ```sh
//! cargo run task
//! ```
//!
//! To override existing data and reset the data structure, use the following
//! command with the `refresh:true` argument:
//! ```sh
//! cargo run task seed_data refresh:true
//! ```

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use loco_rs::{db, prelude::*};

use crate::{app::App, workers::crawler::{CrawlerWorker, CrawlerWorkerArgs}};

const DOMAINS_PATH: &str = "assets/domains.txt";

#[allow(clippy::module_name_repetitions)]
pub struct EnqueueCrawler;
#[async_trait]
impl Task for EnqueueCrawler {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "enqueue_crawler".to_string(),
            detail: "Task for seeding data".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        // Read domains from file
        let default_path = String::from(DOMAINS_PATH);
        let path = vars
            .cli_arg("path")
            .unwrap_or(&default_path); 

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let domain = line.map_err(|e| Error::string(&e.to_string()))?;
            CrawlerWorker::perform_later(
                &app_context,
                CrawlerWorkerArgs {
                    url: domain,
                },
            )
            .await?;

            //return Ok(())
        }

        Ok(())
    }
}
