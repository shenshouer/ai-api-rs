use axum::{response::IntoResponse, routing::get, Router};
use tracing::{debug, instrument};

use crate::api::{observe::RequestContext, routes::AppState, ApiResponse};

pub fn routes() -> Router<AppState> {
    Router::new().route("/hello", get(hello_world))
}

#[instrument(level = "info", skip(context))]
async fn hello_world(context: RequestContext) -> impl IntoResponse {
    debug!("Hello, World!");
    ApiResponse::success(context, "Hello, World!")
}
