use axum::{
    extract::Request,
    http,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use tracing::info;

pub async fn logger_middleware(request: Request, next: Next) -> Response {
    if let Some(path) = request.uri().path_and_query() {
        info!(target: "access_log", "Accessed route: {}", path)
    }
    next.run(request).await
}
