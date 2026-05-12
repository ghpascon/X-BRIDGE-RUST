use std::sync::Arc;

use axum::{Extension, Router, routing::get};
use minijinja::Environment;

use crate::{docs, routers};

type AppResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn build_app() -> AppResult<Router> {
    let templates = Arc::new(routers::pages::templates::load_templates()?);

    let api_router = routers::api::register_modules(Router::new(), routers::api::all_modules());

    Ok(Router::new()
        .route("/", get(routers::pages::home::home))
        .merge(routers::pages::static_files::router())
        .merge(api_router)
        .merge(docs::router())
        .layer(Extension::<Arc<Environment<'static>>>(templates)))
}
