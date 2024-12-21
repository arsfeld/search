use loco_rs::prelude::*;

use crate::models::_entities::{pages, websites};

/// Render a list view of websites.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(
    v: &impl ViewRenderer,
    items: &Vec<(websites::Model, Vec<pages::Model>)>,
) -> Result<Response> {
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

/// Render a website crawling confirmation.
///
/// Render a website crawling confirmation.
///
pub fn crawl_confirm() -> Result<Response> {
    format::html(r#"<i class="fas fa-check text-white text-lg"></i>"#)
}
