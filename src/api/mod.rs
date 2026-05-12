use axum::Router;

pub mod health;
pub mod users;

pub struct ApiModule {
    pub name: &'static str,
    pub router: Router,
}

impl ApiModule {
    #[must_use]
    pub fn new(name: &'static str, router: Router) -> Self {
        Self { name, router }
    }

    #[must_use]
    pub fn mount_path(&self) -> String {
        format!("/api/v1/{}", self.name)
    }
}

#[must_use]
pub fn all_modules() -> Vec<ApiModule> {
    vec![health::module(), users::module()]
}

#[must_use]
pub fn register_modules(base_router: Router, modules: Vec<ApiModule>) -> Router {
    modules.into_iter().fold(base_router, |router, module| {
        router.nest(&module.mount_path(), module.router)
    })
}

#[cfg(test)]
mod tests {
    use axum::{Router, body::Body, http::Request};
    use tower::ServiceExt;

    use super::{health, register_modules};

    #[tokio::test]
    async fn modules_are_automatically_prefixed_with_api_v1_and_module_name() {
        let app = register_modules(Router::new(), vec![health::module()]);

        let prefixed = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/v1/health/ping")
                    .body(Body::empty())
                    .expect("request should build"),
            )
            .await
            .expect("request should succeed");
        assert_eq!(prefixed.status(), 200);

        let unprefixed = app
            .oneshot(
                Request::builder()
                    .uri("/health/ping")
                    .body(Body::empty())
                    .expect("request should build"),
            )
            .await
            .expect("request should succeed");
        assert_eq!(unprefixed.status(), 404);
    }
}
