use axum::{Json, Router, routing::get};

use crate::schemas::health::HealthResponse;
use crate::services::health_service::HealthService;

use super::ApiModule;

#[utoipa::path(
    get,
    path = "/api/v1/health/ping",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
pub async fn ping() -> Json<HealthResponse> {
    let status = HealthService::check();
    Json(HealthResponse {
        status: status.status,
    })
}

#[must_use]
pub fn module() -> ApiModule {
    ApiModule::new("health", Router::new().route("/ping", get(ping)))
}
