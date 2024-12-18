use loco_rs::prelude::*;

use crate::models::_entities::pages;

/// Render a list view of pages.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<pages::Model>) -> Result<Response> {
    format::render().view(v, "page/list.html", data!({"items": items}))
}

/// Render a single page view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &pages::Model) -> Result<Response> {
    format::render().view(v, "page/show.html", data!({"item": item}))
}

/// Render a page create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "page/create.html", data!({}))
}

/// Render a page edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &pages::Model) -> Result<Response> {
    format::render().view(v, "page/edit.html", data!({"item": item}))
}
