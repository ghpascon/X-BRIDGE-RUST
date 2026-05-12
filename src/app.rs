use std::{path::Path, sync::Arc};

use axum::{
    Extension, Router,
    routing::{get, get_service},
};
use minijinja::Environment;
use tower_http::services::ServeDir;

use crate::{api, docs, web};

type AppResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn build_app() -> AppResult<Router> {
    let templates = Arc::new(web::templates::load_templates(Path::new("templates"))?);

    let api_router = api::register_modules(Router::new(), api::all_modules());

    Ok(Router::new()
        .route("/", get(web::home::home))
        .nest_service("/static", get_service(ServeDir::new("static")))
        .merge(api_router)
        .merge(docs::router())
        .layer(Extension::<Arc<Environment<'static>>>(templates)))
}
