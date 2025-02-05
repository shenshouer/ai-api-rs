use axum::response::IntoResponse;
use tracing::{info, instrument};

use crate::api::{observe::RequestContext, ApiResponse};

#[instrument]
pub async fn check(mut context: RequestContext) -> impl IntoResponse {
    context.generate_trace_id();
    let tracer_id = context.trace_id.as_ref().unwrap();

    info!(trace.id = %tracer_id, "health check");
    ApiResponse::success(context, ())
}
