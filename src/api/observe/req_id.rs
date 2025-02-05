use axum::http::{HeaderValue, Request};
use tower_http::request_id::{MakeRequestId, RequestId};

#[derive(Clone, Default)]
pub struct MakeRequestUuid;

/// 默认从请求中获取x-request-id，如果没有则生成一个uuid
impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, request: &Request<B>) -> Option<RequestId> {
        let request_id = request
            .headers()
            .get("x-request-id")
            .map_or(uuid::Uuid::new_v4().to_string(), |v| {
                v.to_str().unwrap().to_string()
            });

        Some(RequestId::new(
            HeaderValue::from_bytes(request_id.as_bytes()).unwrap(),
        ))
    }
}
