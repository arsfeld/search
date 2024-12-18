use std::collections::HashMap;

use axum::{async_trait, Extension, Router as AxumRouter};
use fluent_templates::{ArcLoader, FluentLoader};
use loco_rs::{
    app::{AppContext, Initializer},
    controller::views::{engines, ViewEngine},
    Error, Result,
};
use serde_json::{to_value, Value};
use tracing::info;

const I18N_DIR: &str = "assets/i18n";
const I18N_SHARED: &str = "assets/i18n/shared.ftl";
#[allow(clippy::module_name_repetitions)]
pub struct ViewEngineInitializer;

fn snippet(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    let text = value.to_string();
    let query = args
        .get("query")
        .unwrap_or(&Value::String("".to_string()))
        .to_string();
    let context_size = args
        .get("context_size")
        .unwrap_or(&Value::from(200))
        .as_u64()
        .unwrap() as usize;

    if let Some(start_index) = text.to_lowercase().find(&query.to_lowercase()) {
        info!("Found query in start index: {}", start_index);
        let start = start_index.saturating_sub(context_size);
        let end = (start_index + query.len() + context_size).min(text.len());

        let mut snippet = String::new();
        if start > 0 {
            snippet.push_str("...");
        }
        snippet.push_str(&text[start..end]);
        if end < text.len() {
            snippet.push_str("...");
        }
        Ok(to_value(snippet).unwrap())
    } else {
        // If query not found, return the first few characters
        Ok(to_value(text.chars().take(context_size * 2).collect::<String>() + "...").unwrap())
    }
}

#[async_trait]
impl Initializer for ViewEngineInitializer {
    fn name(&self) -> String {
        "view-engine".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        #[allow(unused_mut)]
        let mut tera_engine = engines::TeraView::build()?;
        if std::path::Path::new(I18N_DIR).exists() {
            let arc = ArcLoader::builder(&I18N_DIR, unic_langid::langid!("en-US"))
                .shared_resources(Some(&[I18N_SHARED.into()]))
                .customize(|bundle| bundle.set_use_isolating(false))
                .build()
                .map_err(|e| Error::string(&e.to_string()))?;
            #[cfg(debug_assertions)]
            tera_engine
                .tera
                .lock()
                .expect("lock")
                .register_function("t", FluentLoader::new(arc));

            #[cfg(not(debug_assertions))]
            tera_engine
                .tera
                .register_function("t", FluentLoader::new(arc));
            info!("locales loaded");
        }

        tera_engine
            .tera
            .lock()
            .expect("lock")
            .register_filter("snippet", snippet);

        Ok(router.layer(Extension(ViewEngine::from(tera_engine))))
    }
}
