use axum::Router;
use tower_http::services::ServeDir;

/// Serves everything under `src/static/` at `/static/*`.
#[must_use]
pub fn router() -> Router {
    Router::new().nest_service("/static", ServeDir::new("src/static"))
}
