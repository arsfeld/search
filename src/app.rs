use async_trait::async_trait;
use axum::Extension;
use lazy_static::lazy_static;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    bgworker::{BackgroundWorker, Queue},
    boot::{create_app, BootResult, StartMode},
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    task::Tasks,
    Result,
};
use migration::Migrator;
use sea_orm::DatabaseConnection;
use std::{
    path::Path,
    sync::{Arc, RwLock},
};
use tantivy::{
    directory::MmapDirectory,
    schema::{Schema, STORED, TEXT},
    Directory, Index, IndexReader, IndexWriter, ReloadPolicy,
};

use crate::{
    controllers, initializers,
    models::{_entities::users, _entities::websites},
    tasks,
    workers::{crawler::CrawlerWorker, downloader::DownloadWorker},
};

const INDEX_PATH: &str = "data/tantivy";

// Taken from https://github.com/loco-rs/shared-global-state
// Feels a bit weird, probably Loco could offer a better way to share state between different components.
lazy_static! {
    pub static ref tantivy_index: Arc<Index> = {
        tracing::debug!("Initializing Tantivy index");

        // Create data dir if it doesnt exist
        std::fs::create_dir_all(INDEX_PATH).unwrap_or_default();

        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("title", TEXT | STORED);
        schema_builder.add_text_field("url", TEXT | STORED);
        schema_builder.add_text_field("body", TEXT | STORED);
        let schema = schema_builder.build();

        let directory: Box<dyn Directory> = Box::new(MmapDirectory::open(&INDEX_PATH).unwrap());

        let index = Index::open_or_create(directory, schema.clone()).unwrap();

        Arc::new(index)
    };
}

lazy_static! {
    pub static ref tantivy_writer: Arc<RwLock<IndexWriter>> = {
        tracing::debug!("Initializing Tantivy writer");

        Arc::new(RwLock::new(tantivy_index.writer(50_000_000).unwrap()))
    };
}

lazy_static! {
    pub static ref tantivy_reader: Arc<IndexReader> = {
        tracing::debug!("Initializing Tantivy reader");

        Arc::new(
            tantivy_index
                .reader_builder()
                .reload_policy(ReloadPolicy::OnCommitWithDelay)
                .try_into()
                .unwrap(),
        )
    };
}

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(mode: StartMode, environment: &Environment) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![Box::new(
            initializers::view_engine::ViewEngineInitializer,
        )])
    }

    async fn after_routes(router: axum::Router, _ctx: &AppContext) -> Result<axum::Router> {
        Ok(router.layer(Extension(tantivy_index.clone())))
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .add_route(controllers::page::routes())
            .add_route(controllers::website::routes())
            .add_route(controllers::search::routes())
    }

    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        queue
            .register(crate::workers::crawl_all::CrawlAllWorker::build(ctx))
            .await?;
        queue.register(DownloadWorker::build(ctx)).await?;
        queue.register(CrawlerWorker::build(ctx)).await?;
        Ok(())
    }
    fn register_tasks(tasks: &mut Tasks) {
        tasks.register(tasks::seed::SeedData);
        tasks.register(tasks::crawl::Crawl);
        // tasks-inject (do not remove)
    }
    async fn truncate(db: &DatabaseConnection) -> Result<()> {
        truncate_table(db, users::Entity).await?;
        Ok(())
    }

    async fn seed(db: &DatabaseConnection, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(db, &base.join("users.yaml").display().to_string()).await?;
        db::seed::<websites::ActiveModel>(db, &base.join("websites.yaml").display().to_string())
            .await?;
        Ok(())
    }
}
