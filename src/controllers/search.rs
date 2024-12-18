#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::{Deserialize, Serialize};
use tantivy::{collector::TopDocs, query::QueryParser, TantivyDocument};

use tantivy::schema::*;

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

            // info!("Retrieved document: {:?}", retrieved_doc.to_json(&tantivy_index.schema()));

            let full_body = get_string_field(&retrieved_doc, body);

            // Convert the retrieved fields into a ResultItem
            ResultItem {
                title: get_string_field(&retrieved_doc, title),
                url: get_string_field(&retrieved_doc, url),
                body: full_body.clone(),
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
