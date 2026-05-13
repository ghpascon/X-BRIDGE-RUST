use std::sync::Arc;

use axum::{
    Extension,
    response::{Html, IntoResponse},
};
use minijinja::{Environment, context};

use crate::core::error::AppError;

pub async fn home(
    Extension(templates): Extension<Arc<Environment<'static>>>,
) -> Result<impl IntoResponse, AppError> {
    let template = templates
        .get_template("/index/main.html")
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let html = template
        .render(context! {
            title => "X-BRIDGE-RUST",
            heading => "X-BRIDGE-RUST API",
            message => "Use /docs for Swagger and /api/v1/* for API modules."
        })
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Html(html))
}
