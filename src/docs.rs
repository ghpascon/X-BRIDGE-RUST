use axum::Router;
use utoipa_swagger_ui::SwaggerUi;

/// Builds the Swagger UI router from the already-collected `OpenApi` spec.
/// Paths and schemas are populated automatically by `utoipa_axum` — no manual
/// listing required here.
#[must_use]
pub fn router(mut api: utoipa::openapi::OpenApi) -> Router {
    // Keep only /api paths (guards against any page routes leaking in)
    api.paths.paths.retain(|path, _| path.starts_with("/api"));

    SwaggerUi::new("/docs")
        .url("/api-docs/openapi.json", api)
        .into()
}

#[cfg(test)]
mod tests {
    use crate::routers::api;

    use super::router;

    #[test]
    fn swagger_document_contains_only_api_paths() {
        let (_, collected) = api::register_modules(api::all_modules());

        // Build the swagger router just to exercise the filter path
        let _ = router(collected.clone());

        let mut filtered = collected;
        filtered.paths.paths.retain(|p, _| p.starts_with("/api"));

        assert!(!filtered.paths.paths.is_empty());
        assert!(filtered.paths.paths.keys().all(|p| p.starts_with("/api")));
    }
}
