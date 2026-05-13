use axum::{Json, http::StatusCode};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{schemas::users::UserSummary, services::user_service::UserService};

use super::ApiModule;

/// Lists all users.
#[utoipa::path(
    get,
    path = "/",
    tag = "users",
    responses(
        (status = 200, description = "List of users", body = Vec<UserSummary>)
    )
)]
pub async fn list_users() -> (StatusCode, Json<Vec<UserSummary>>) {
    let users = UserService::list()
        .into_iter()
        .map(|u| UserSummary {
            id: u.id,
            name: u.name,
        })
        .collect();
    (StatusCode::OK, Json(users))
}

#[must_use]
pub fn module() -> ApiModule {
    ApiModule::new("users", OpenApiRouter::new().routes(routes!(list_users)))
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    use crate::routers::api::register_modules;

    use super::module;

    #[tokio::test]
    async fn list_users_returns_200() {
        let (app, _) = register_modules(vec![module()]);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/users")
                    .body(Body::empty())
                    .expect("request should build"),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), 200);
    }
}
