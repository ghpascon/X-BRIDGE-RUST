use axum::{Json, Router, routing::get};

use crate::schemas::users::UserSummary;
use crate::services::user_service::UserService;

use super::ApiModule;

#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "users",
    responses(
        (status = 200, description = "List users", body = [UserSummary])
    )
)]
pub async fn list_users() -> Json<Vec<UserSummary>> {
    let users = UserService::list();
    Json(
        users
            .into_iter()
            .map(|u| UserSummary { id: u.id, name: u.name })
            .collect(),
    )
}

#[must_use]
pub fn module() -> ApiModule {
    ApiModule::new("users", Router::new().route("/", get(list_users)))
}
