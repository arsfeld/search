use loco_rs::prelude::*;
use loco_rs::testing;
use search::app::App;

use search::workers::crawl_all::CrawlAllWorker;
use search::workers::crawl_all::CrawlAllWorkerArgs;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_run_crawl_all_worker() {
    let boot = testing::boot_test::<App>().await.unwrap();

    // Execute the worker ensuring that it operates in 'ForegroundBlocking' mode, which prevents the addition of your worker to the background
    assert!(
        CrawlAllWorker::perform_later(&boot.app_context, CrawlAllWorkerArgs {})
            .await
            .is_ok()
    );
    // Include additional assert validations after the execution of the worker
}
