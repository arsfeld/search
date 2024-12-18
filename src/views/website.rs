use loco_rs::prelude::*;

use crate::models::_entities::websites;

/// Render a list view of websites.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<websites::Model>) -> Result<Response> {
    format::render().view(v, "website/list.html", data!({"items": items}))
}

/// Render a single website view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &websites::Model) -> Result<Response> {
    format::render().view(v, "website/show.html", data!({"item": item}))
}

/// Render a website create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "website/create.html", data!({}))
}

/// Render a website edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &websites::Model) -> Result<Response> {
    format::render().view(v, "website/edit.html", data!({"item": item}))
}
