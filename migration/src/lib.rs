#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;

mod m20241218_025604_htmx_tests;
mod m20241218_155733_stats;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20241218_025604_htmx_tests::Migration),
            Box::new(m20241218_155733_stats::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}