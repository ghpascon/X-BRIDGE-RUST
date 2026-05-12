use axum::{
    Router,
    body::Body,
    http::{StatusCode, header},
    response::Response,
    routing::get,
};

/// CSS bundle embedded in the binary at compile time.
static APP_CSS: &[u8] = include_bytes!("../../static/css/app.css");

pub async fn serve_app_css() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/css; charset=utf-8")
        .header(header::CACHE_CONTROL, "public, max-age=31536000, immutable")
        .body(Body::from(APP_CSS))
        .expect("static css response should build")
}

/// Router that serves all embedded static assets.
#[must_use]
pub fn router() -> Router {
    Router::new().route("/static/css/app.css", get(serve_app_css))
}
