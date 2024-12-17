use std::sync::Arc;

use axum::{async_trait, Extension, Router as AxumRouter};
use loco_rs::{
    app::{AppContext, Initializer},
    Error, Result,
};
use tantivy::IndexWriter;

use crate::app::tantivy_index;

#[allow(clippy::module_name_repetitions)]
pub struct TantivyWriterInitializer;

#[async_trait]
impl Initializer for TantivyWriterInitializer {
    fn name(&self) -> String {
        "tantivy_writer".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let index_writer: IndexWriter = tantivy_index.writer(50_000_000)
            .map_err(|e| Error::Message(e.to_string()))?;

        Ok(router.layer(Extension(Arc::new(index_writer))))
    }
}
