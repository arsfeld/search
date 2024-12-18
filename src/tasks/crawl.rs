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
};

use loco_rs::prelude::*;

use crate::workers::crawler::{CrawlerWorker, CrawlerWorkerArgs};

const DOMAINS_PATH: &str = "assets/domains.txt";

#[allow(clippy::module_name_repetitions)]
pub struct Crawl;
#[async_trait]
impl Task for Crawl {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "crawl".to_string(),
            detail: "Task for seeding data".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        // Read domains from file
        let default_path = String::from(DOMAINS_PATH);
        let path = vars.cli_arg("path").unwrap_or(&default_path);

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for (i, line) in reader.lines().enumerate() {
            let domain = line.map_err(|e| Error::string(&e.to_string()))?;
            CrawlerWorker::perform_later(&app_context, CrawlerWorkerArgs { url: domain }).await?;

            // Uncomment this to limit the number of tasks
            if i >= 5 {
                break;
            }
            //return Ok(())
        }

        Ok(())
    }
}
