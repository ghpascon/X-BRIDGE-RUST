use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::{health, users};

#[derive(OpenApi)]
#[openapi(
    paths(
        health::ping,
        users::list_users
    ),
    components(
        schemas(
            health::HealthResponse,
            users::UserSummary
        )
    ),
    tags(
        (name = "health", description = "Health endpoints"),
        (name = "users", description = "User endpoints")
    )
)]
struct ApiDoc;

#[must_use]
pub fn openapi_document() -> utoipa::openapi::OpenApi {
    let mut document = ApiDoc::openapi();
    filter_api_only_paths(&mut document);
    document
}

fn filter_api_only_paths(document: &mut utoipa::openapi::OpenApi) {
    document
        .paths
        .paths
        .retain(|path, _| path.starts_with("/api"));
}

#[must_use]
pub fn router() -> Router {
    SwaggerUi::new("/docs")
        .url("/api-docs/openapi.json", openapi_document())
        .into()
}

#[cfg(test)]
mod tests {
    use super::openapi_document;

    #[test]
    fn swagger_document_contains_only_api_paths() {
        let document = openapi_document();
        assert!(!document.paths.paths.is_empty());
        assert!(
            document
                .paths
                .paths
                .keys()
                .all(|path| path.starts_with("/api"))
        );
    }
}
