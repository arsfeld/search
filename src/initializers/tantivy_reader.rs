
use axum::{async_trait, Extension, Router as AxumRouter};
use loco_rs::{
    app::{AppContext, Initializer},
    Error, Result,
};
use tantivy::ReloadPolicy;

use crate::app::tantivy_index;

#[allow(clippy::module_name_repetitions)]
pub struct TantivyReaderInitializer;

#[async_trait]
impl Initializer for TantivyReaderInitializer {
    fn name(&self) -> String {
        "tantivy_reader".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        // Get tantivy index from router
        let reader = tantivy_index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .map_err(|e| Error::Message(e.to_string()))?;

        Ok(router.layer(Extension(reader)))
    }
}
