use crate::handlers::storage_handler::{get_storage_router, make_storage_router};

use axum::middleware::from_fn;
use axum::Router;
use db::context::RouterCtx;
use db::prisma;
use db::prisma::settings;
use db::prisma::PrismaClient;
use handlers::translation_handler::*;
use local_storage::stores::location_store::LocationStore;
use local_storage::stores::settings_store::SettingsStore;
use local_storage::stores::translation_store::TranslationStore;
use rspc::internal::specta;
use rspc::{Config, Router as RspcRouter};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use specta::Type;
use std::sync::Arc;
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
        .config(Config::new().export_ts_bindings("../../ui/lib/procedures.ts"))
        .query("hi", |t| t(|ctx, input: ()| "hello world"))
        .merge("stores.", get_storage_router())
        .build()
}
