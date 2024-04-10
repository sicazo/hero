use crate::handlers::location_handler::get_location_router;
use axum::middleware::from_fn;
use axum::Router;
use db::context::RouterCtx;
use handlers::{storage_handler::get_storage_router, translation_handler::*};
use rspc::{Config, Router as RspcRouter};
use tokio::time::{sleep, Duration};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

mod handlers;
mod own_middleware;

#[tokio::main]
pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    // CORS Setup
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any)
        .allow_credentials(false);

    let app = Router::new()
        // .route("/graphql_ws", get(graphql_ws_handler))
        .layer(cors)
        .layer(from_fn(own_middleware::logger::logger_middleware));

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

pub fn get_router() -> RspcRouter<RouterCtx> {
    RspcRouter::<RouterCtx>::new()
        .config(Config::new().export_ts_bindings("../../ui/lib/procedures.ts"))
        .query("hi", |t| t(|_ctx, _input: ()| "hello world"))
        .merge("stores.", get_storage_router())
        .merge("translations.", get_translation_router())
        .merge("locations.", get_location_router())
        .subscription("test", |t| {
            t(|ctx, input: ()| {
                async_stream::stream! {
                    for i in 0..5 {
                        yield "ping".to_string();
                        sleep(Duration::from_secs(1)).await;
                    }
                }
            })
        })
        .build()
}
