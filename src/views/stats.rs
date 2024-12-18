use loco_rs::prelude::*;

use crate::models::_entities::stats;

/// Render a list view of stats.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<stats::Model>) -> Result<Response> {
    format::render().view(v, "stats/list.html", data!({"items": items}))
}

/// Render a single stats view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &stats::Model) -> Result<Response> {
    format::render().view(v, "stats/show.html", data!({"item": item}))
}

/// Render a stats create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "stats/create.html", data!({}))
}

/// Render a stats edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &stats::Model) -> Result<Response> {
    format::render().view(v, "stats/edit.html", data!({"item": item}))
}
