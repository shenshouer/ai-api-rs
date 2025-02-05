use axum::{http::StatusCode, response::IntoResponse, Json};
use tracing::debug;

use super::{observe::RequestContext, resp::ApiResponse};

pub struct ApiError {
    err: Error,
    context: RequestContext,
}

impl ApiError {
    pub fn new(err: Error, context: RequestContext) -> Self {
        Self { err, context }
    }
}

pub enum Error {
    NotFound(String),
    BadRequest(String),
    InternalError(String),
    Unauthorized(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        use Error::*;
        let result = match self.err {
            NotFound(msg) => (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::<()>::error(self.context, 404, msg)),
            ),
            BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error(self.context, 400, msg)),
            ),
            InternalError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(self.context, 500, msg)),
            ),
            Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error(self.context, 401, msg)),
            ),
        };
        result.into_response()
    }
}

pub type Result<T> = std::result::Result<T, ApiError>;
