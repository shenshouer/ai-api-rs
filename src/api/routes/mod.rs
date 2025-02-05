use std::{sync::Arc, time::Duration};

use crate::api::error::Error;

use super::{
    error::{ApiError, Result},
    observe::RequestContext,
};
use axum::{
    body::Bytes,
    http::{header, Request},
    routing::get,
    Router,
};
use healthz::check;
use log::warn;
use opentelemetry::trace::TraceContextExt;
use tower::ServiceBuilder;
use tower_http::{
    classify::StatusInRangeAsFailures,
    cors::{Any, CorsLayer},
    request_id::MakeRequestUuid,
    timeout::TimeoutLayer,
    trace::{DefaultOnResponse, TraceLayer},
    LatencyUnit, ServiceBuilderExt,
};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use uuid::Uuid;

mod api;
mod healthz;

#[derive(Clone)]
pub struct AppState {}

pub fn routes() -> Router {
    let state = AppState {};
    let sensitive_headers: Arc<[_]> = vec![header::AUTHORIZATION, header::COOKIE].into();
    // x-request-id
    let middleware = ServiceBuilder::new()
        .sensitive_request_headers(sensitive_headers.clone())
        .set_x_request_id(MakeRequestUuid)
        .propagate_x_request_id()
        .layer(
            TraceLayer::new(StatusInRangeAsFailures::new(400..=599).into_make_classifier())
                .on_body_chunk(|chunk: &Bytes, latency: Duration, _: &tracing::Span| {
                    tracing::trace!(size_bytes = chunk.len(), latency = ?latency, "sending body chunk")
                })
                .make_span_with(|req: &Request<_>| {
                    // 获取或生成Request ID
                    let request_id = req.headers().get("x-request-id").map_or_else(
                        || Uuid::new_v4().to_string(),
                        |id| id.to_str().unwrap().to_string(),
                    );
                    // 创建Span
                    let span = tracing::info_span!(
                        "",
                        method = %req.method(),
                        uri = %req.uri(),
                        rid = %request_id,
                        tid = tracing::field::Empty,
                    );
                    // 记录Trace ID
                    {
                        let _enter = span.enter();
                        span.record("tid", tracing::Span::current().context().span().span_context().trace_id().to_string());
                    }
                    span
                })
                .on_response(DefaultOnResponse::new().include_headers(true).latency_unit(LatencyUnit::Micros)),
        )
        .sensitive_response_headers(sensitive_headers)
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .compression();
    // 跨域
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    Router::new()
        .nest("/api", api::routes())
        .route("/healthz", get(check))
        .with_state(state)
        .fallback(not_found)
        .layer(cors)
        .layer(middleware)
        .layer(TraceLayer::new_for_http())
}

async fn not_found(context: RequestContext) -> Result<()> {
    warn!("not_found {context:?}");
    Err(ApiError::new(
        Error::NotFound("not fountd".to_owned()),
        context,
    ))
}
