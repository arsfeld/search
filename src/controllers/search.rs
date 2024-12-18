#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use sea_orm::{sea_query::Order, QueryOrder};
use axum::debug_handler;
use tantivy::{collector::TopDocs, query::QueryParser, TantivyDocument};

use tantivy::schema::*;
use tracing::info;

use crate::models::search::ResultItem;
use crate::{
    app::{tantivy_index, tantivy_reader}, models::_entities::htmx_tests::{ActiveModel, Column, Entity, Model}, views
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

#[debug_handler]
pub async fn results(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
    Form(params): Form<Params>,
) -> Result<Response> {
    let searcher = tantivy_reader.searcher();

    let title = tantivy_index.schema().get_field("title").unwrap();
    let url = tantivy_index.schema().get_field("url").unwrap();
    let body = tantivy_index.schema().get_field("body").unwrap();
    
    let query_parser = QueryParser::for_index(&tantivy_index, vec![title, body, url]);
    let query = query_parser.parse_query(&params.query).unwrap();
    
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10)).unwrap();
    
    let results = top_docs
        .into_iter()
        .map(|(_score, doc_address)| {
            let retrieved_doc: TantivyDocument = searcher.doc(doc_address).unwrap();

            info!("Retrieved document: {:?}", retrieved_doc.to_json(&tantivy_index.schema()));

            // Convert the retrieved fields into a ResultItem
            ResultItem {
                title: retrieved_doc.get_first(title)
                    .and_then(|v| match v.to_owned() {
                        OwnedValue::Str(s) => Some(s),
                        _ => Some("".to_string()),
                    })
                    .unwrap_or_default(),
                url: retrieved_doc.get_first(url)
                    .and_then(|v| match v.to_owned() {
                        OwnedValue::Str(s) => Some(s),
                        _ => Some("".to_string()),
                    })
                    .unwrap_or_default(),
                body: retrieved_doc.get_first(body)
                    .and_then(|v| match v.to_owned() {
                        OwnedValue::Str(s) => Some(s),
                        _ => Some("".to_string()),
                    })
                    .unwrap_or_default(),
            }
        })
        .collect();

    views::search::results(&v, &params.query, results)
}

pub fn routes() -> Routes {
    Routes::new()
        //.prefix("/")
        .add("/", get(index))
        .add("/search", post(results))
}
