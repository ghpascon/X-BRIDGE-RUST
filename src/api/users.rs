use axum::{Json, Router, routing::get};
use serde::Serialize;
use utoipa::ToSchema;

use super::ApiModule;

#[derive(Serialize, ToSchema)]
pub struct UserSummary {
    pub id: u64,
    pub name: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "users",
    responses(
        (status = 200, description = "List users", body = [UserSummary])
    )
)]
pub async fn list_users() -> Json<Vec<UserSummary>> {
    Json(vec![UserSummary {
        id: 1,
        name: "Admin".to_string(),
    }])
}

#[must_use]
pub fn module() -> ApiModule {
    ApiModule::new("users", Router::new().route("/", get(list_users)))
}
