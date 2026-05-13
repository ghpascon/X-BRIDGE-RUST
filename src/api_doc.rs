use utoipa::OpenApi;

/// Base OpenAPI document — defines metadata and tags.
/// Paths and schemas are collected automatically from each route module
/// via `utoipa_axum`'s `OpenApiRouter` + `routes!` macro.
#[derive(OpenApi)]
#[openapi(
    info(title = "X-BRIDGE-RUST", version = "0.1.2"),
    tags(
        (name = "health", description = "Health endpoints"),
        (name = "users",  description = "User endpoints")
    )
)]
pub struct ApiDoc;
