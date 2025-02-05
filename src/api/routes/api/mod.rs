mod v1;

use axum::Router;

use super::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/v1", v1::routes())
}
