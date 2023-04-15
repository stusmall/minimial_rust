use axum::body::Empty;
use axum::response::{IntoResponse, Response};
use  tracing::instrument;

#[instrument(level = "debug")]
pub async fn health_check() -> Response {
    Empty::new().into_response()
}