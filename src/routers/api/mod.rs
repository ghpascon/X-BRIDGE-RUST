use axum::Router;
use utoipa::OpenApi as _;
use utoipa::openapi::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::api_doc::ApiDoc;

pub mod health;

pub struct ApiModule {
    pub name: &'static str,
    pub router: OpenApiRouter,
}

impl ApiModule {
    #[must_use]
    pub fn new(name: &'static str, router: OpenApiRouter) -> Self {
        Self { name, router }
    }

    #[must_use]
    pub fn mount_path(&self) -> String {
        format!("/api/v1/{}", self.name)
    }
}

#[must_use]
pub fn all_modules() -> Vec<ApiModule> {
    vec![health::module()]
}

/// Registers all modules and returns the merged axum `Router` together with
/// the fully-collected `OpenApi` spec — paths and schemas are picked up
/// automatically from each handler's `#[utoipa::path]` annotation via
/// `utoipa_axum`.
#[must_use]
pub fn register_modules(modules: Vec<ApiModule>) -> (Router, OpenApi) {
    modules
        .into_iter()
        .fold(
            OpenApiRouter::with_openapi(ApiDoc::openapi()),
            |acc, module| acc.nest(&module.mount_path(), module.router),
        )
        .split_for_parts()
}
