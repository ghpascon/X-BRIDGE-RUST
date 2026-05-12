use std::sync::Arc;

use axum::{
    Extension,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use minijinja::{Environment, context};

pub async fn home(
    Extension(templates): Extension<Arc<Environment<'static>>>,
) -> Result<impl IntoResponse, StatusCode> {
    let template = templates
        .get_template("home.html")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let html = template
        .render(context! {
            title => "X-BRIDGE-RUST",
            heading => "X-BRIDGE-RUST API",
            message => "Use /docs for Swagger and /api/v1/* for API modules."
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Html(html))
}
