use axum::{extract::FromRequestParts, http::request::Parts};
use opentelemetry::trace::TraceContextExt;
use serde::{Deserialize, Serialize};
use tower_http::request_id::RequestId;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestContext {
    pub uri: String,              // 请求路径和查询参数
    pub request_id: String,       // 请求唯一ID
    pub trace_id: Option<String>, // 跟踪ID（示例使用字符串）
}

impl RequestContext {
    pub fn generate_trace_id(&mut self) {
        self.trace_id = Some(
            tracing::Span::current()
                .context()
                .span()
                .span_context()
                .trace_id()
                .to_string(),
        );
    }
}

impl<S> FromRequestParts<S> for RequestContext
where
    S: Send + Sync,
{
    type Rejection = axum::response::Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 生成请求ID（示例使用UUID，实际可从中间件获取）
        let request_id = parts
            .extensions
            .get::<RequestId>()
            .and_then(|id| id.header_value().to_str().ok().map(|s| s.to_string()))
            .unwrap_or(Uuid::new_v4().to_string());

        // 此处在Span之前，无法获取到trace_id
        // let trace_id = tracing::Span::current()
        //     .context()
        //     .span()
        //     .span_context()
        //     .trace_id()
        //     .to_string();
        Ok(Self {
            uri: parts.uri.clone().to_string(),
            request_id,
            trace_id: None,
        })
    }
}
