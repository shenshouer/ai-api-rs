use axum::response::IntoResponse;
use tracing::{info, instrument};

use crate::api::{observe::RequestContext, ApiResponse};

#[instrument(level = "info", skip(context))]
pub async fn check(context: RequestContext) -> impl IntoResponse {
    info!(trace.id = %context.trace_id, "health check");
    ApiResponse::success(context, ())
}
