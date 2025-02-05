use std::{sync::Arc, time::Duration};

use crate::api::error::Error;

use super::{
    error::{ApiError, Result},
    observe::{MakeRequestUuid, RequestContext},
};
use axum::{http::header, routing::get, Router};
use healthz::check;
use log::warn;
use tower::ServiceBuilder;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer, ServiceBuilderExt};

mod api;
mod healthz;

#[derive(Clone)]
pub struct AppState {}

pub fn routes() -> Router {
    let state = AppState {};
    let sensitive_headers: Arc<[_]> = vec![header::AUTHORIZATION, header::COOKIE].into();
    let middleware = ServiceBuilder::new()
        .sensitive_request_headers(sensitive_headers.clone())
        .set_x_request_id(MakeRequestUuid)
        .propagate_x_request_id()
        .sensitive_response_headers(sensitive_headers)
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .compression();

    Router::new()
        .with_state(state)
        .route("/healthz", get(check))
        .fallback(not_found)
        .layer(middleware)
        .layer(TraceLayer::new_for_http())
    // .nest("/api", api::routes(state.clone()))
}

async fn not_found(context: RequestContext) -> Result<()> {
    warn!("not_found {context:?}");
    Err(ApiError::new(
        Error::NotFound("not fountd".to_owned()),
        context,
    ))
}
