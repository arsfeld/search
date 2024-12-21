#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use std::sync::Arc;

use axum::debug_handler;
use futures::stream::{self, StreamExt};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use tantivy::{collector::TopDocs, query::QueryParser, TantivyDocument};

use tantivy::schema::*;

use crate::models::_entities::pages::{self};
use crate::models::search::ResultItem;
use crate::{
    app::{tantivy_index, tantivy_reader},
    views,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    query: String,
}

#[debug_handler]
pub async fn index(
    ViewEngine(v): ViewEngine<TeraView>,
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    views::search::index(&v)
}

fn get_string_field(doc: &TantivyDocument, field: Field) -> String {
    doc.get_first(field)
        .and_then(|v| match v {
            OwnedValue::Str(s) => Some(s.to_string()),
            _ => None,
        })
        .unwrap_or_default()
}

#[debug_handler]
pub async fn results(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
    Form(params): Form<Params>,
) -> Result<Response> {
    let start = spider::tokio::time::Instant::now();

    let searcher = Arc::new(tantivy_reader.searcher());

    let title = tantivy_index.schema().get_field("title").unwrap();
    let url = tantivy_index.schema().get_field("url").unwrap();
    let body = tantivy_index.schema().get_field("body").unwrap();

    let query_parser = QueryParser::for_index(&tantivy_index, vec![title, body, url]);
    let query = query_parser.parse_query(&params.query).unwrap();

    let top_docs = searcher.search(&query, &TopDocs::with_limit(10)).unwrap();

    let db = ctx.db.clone();

    let results = stream::iter(top_docs)
        .map(|(_score, doc_address)| {
            let searcher_clone: Arc<tantivy::Searcher> = searcher.clone();
            let db_clone = db.clone();
            async move {
                let retrieved_doc: TantivyDocument = searcher_clone.doc(doc_address).unwrap();

                let item = pages::Entity::find()
                    .filter(pages::Column::Url.eq(get_string_field(&retrieved_doc, url)))
                    .one(&db_clone)
                    .await;

                let _item = match item {
                    Ok(Some(i)) => i,
                    Ok(None) => {
                        // Handle case where no item was found
                        tracing::warn!(
                            "No item found for URL: {}",
                            get_string_field(&retrieved_doc, url)
                        );
                        return None;
                    }
                    Err(e) => {
                        // Handle the error case
                        tracing::error!("Error fetching item from database: {:?}", e);
                        return None;
                    }
                };

                Some(ResultItem {
                    title: get_string_field(&retrieved_doc, title),
                    url: get_string_field(&retrieved_doc, url),
                    body: _item.body,
                })
            }
        })
        .filter_map(|item| async move { item.await }) // Remove None values
        .collect()
        .await;

    let duration = start.elapsed();

    views::search::results(&v, &params.query, results, duration)
}

pub fn routes() -> Routes {
    Routes::new()
        //.prefix("/")
        .add("/", get(index))
        .add("/search", post(results))
}
