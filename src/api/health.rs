use axum::{Json, Router, routing::get};
use serde::Serialize;
use utoipa::ToSchema;

use super::ApiModule;

#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: &'static str,
}

#[utoipa::path(
    get,
    path = "/api/v1/health/ping",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
pub async fn ping() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

#[must_use]
pub fn module() -> ApiModule {
    ApiModule::new("health", Router::new().route("/ping", get(ping)))
}
