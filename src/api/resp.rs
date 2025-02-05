use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use super::observe::RequestContext;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    #[serde(flatten)]
    context: RequestContext,
    code: i32,
    error: Option<String>,
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    /// 创建成功响应
    pub fn success(context: RequestContext, data: T) -> Self {
        Self {
            code: 0,
            data: Some(data),
            error: None,
            context,
        }
    }

    /// 创建错误响应
    pub fn error(context: RequestContext, code: i32, err: String) -> Self {
        Self {
            code,
            error: Some(err),
            data: None,
            context,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageInfo {
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
    pub total_pages: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageResponse<T> {
    pub items: Vec<T>,
    #[serde(flatten)]
    pub page_info: PageInfo,
}

impl<T> ApiResponse<PageResponse<T>> {
    pub fn success_with_page(
        context: RequestContext,
        items: Vec<T>,
        total: i64,
        page: i32,
        page_size: i32,
    ) -> Self {
        let total_pages = ((total as f64) / (page_size as f64)).ceil() as i32;

        Self::success(
            context,
            PageResponse {
                items,
                page_info: PageInfo {
                    total,
                    page,
                    page_size,
                    total_pages,
                },
            },
        )
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        match self.error {
            Some(err) => err.into_response(),
            None => (axum::http::StatusCode::OK, axum::Json(self)).into_response(),
        }
    }
}
