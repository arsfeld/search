use std::time::Duration;

use loco_rs::prelude::*;
use tracing::info;

use crate::models::search::ResultItem;

pub fn index(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "index.html", data!({}))
}

pub fn results(
    v: &impl ViewRenderer,
    query: &str,
    results: Vec<ResultItem>,
    duration: Duration,
) -> Result<Response> {
    info!("Rendering search results for query: {}", query);

    format::render().view(
        v,
        "search/results.html",
        data!({"query": query, "results": results, "duration": duration.as_millis()}),
    )
}
