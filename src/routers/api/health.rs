use axum::{Json, http::StatusCode};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{schemas::health::HealthResponse, services::health_service::HealthService};

use super::ApiModule;

/// Returns 200 OK when the service is running.
#[utoipa::path(
    get,
    path = "/ping",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
pub async fn ping() -> (StatusCode, Json<HealthResponse>) {
    let status = HealthService::check();
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: status.status,
        }),
    )
}

#[must_use]
pub fn module() -> ApiModule {
    ApiModule::new("health", OpenApiRouter::new().routes(routes!(ping)))
}
