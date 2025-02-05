use axum::response::IntoResponse;
use tracing::instrument;

use crate::api::{observe::RequestContext, ApiResponse};

#[instrument(level = "info", skip(context))]
pub async fn check(context: RequestContext) -> impl IntoResponse {
    ApiResponse::success(context, ())
}
