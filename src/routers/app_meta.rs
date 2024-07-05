use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde_json::json;

use crate::AppState;

/// Builds the fallback router.
pub fn build() -> Router<AppState> {
    Router::new()
        .route("/livez", get(livez_handler))
        .route("/readyz", get(readyz_handler))
        .route("/versionz", get(versionz_handler))
}

/// `/livez` handler that always returns a 200
#[axum::debug_handler]
#[tracing::instrument]
async fn livez_handler() -> impl IntoResponse {
    StatusCode::OK
}

/// `/readyz` handler that returns a 200 if everything is good, or a 500
/// otherwise.
#[axum::debug_handler]
#[tracing::instrument]
async fn readyz_handler() -> impl IntoResponse {
    StatusCode::OK
}

/// `/versionz` handler that resturns a JSON object containing this app's
/// version number, and some commit info.
#[axum::debug_handler]
#[tracing::instrument]
async fn versionz_handler() -> impl IntoResponse {
    Json(json!({
        "git": env!("VERGEN_GIT_SHA"),
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
