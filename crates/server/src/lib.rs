use crate::handlers::storage_handler::make_storage_router;

use axum::middleware::from_fn;
use axum::Router;
use handlers::translation_handler::*;
use rspc::{Config, Router as RspcRouter};
use serde_json::Value;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use db::context::RouterCtx;
use db::prisma;
use db::prisma::PrismaClient;
use db::prisma::settings;

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
        .nest("/store", make_storage_router())
        .nest("/translation", make_translation_router())
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
        .config(
            Config::new().export_ts_bindings("../../ui/lib/procedures.ts")
        )

        .query("hi", |t| t(|ctx, input: ()| "hello world"))
        .query("settings", |t| {
            t(|ctx, input: ()| async move {
               let client: &PrismaClient = &ctx.db;
                let settings = client.settings().find_unique(settings::id::equals(1)).exec().await?;
                Ok(settings)
            })
        })
        .build()
}
