#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;

mod m20241218_025604_htmx_tests;
mod m20241218_155733_stats;
mod m20241218_165810_websites;
mod m20241218_174248_pages;
mod m20241218_180428_add_pages_body;
mod m20241218_180815_index_website_domain;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20241218_025604_htmx_tests::Migration),
            Box::new(m20241218_155733_stats::Migration),
            Box::new(m20241218_165810_websites::Migration),
            Box::new(m20241218_174248_pages::Migration),
            Box::new(m20241218_180428_add_pages_body::Migration),
            Box::new(m20241218_180815_index_website_domain::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
