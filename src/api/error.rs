use axum::{http::StatusCode, response::IntoResponse, Json};

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
    #[allow(dead_code)]
    BadRequest(String),
    #[allow(dead_code)]
    Internal(String),
    #[allow(dead_code)]
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
            Internal(msg) => (
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
