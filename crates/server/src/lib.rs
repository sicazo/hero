use crate::entities::application_data;
use crate::handlers::storage_handler::make_storage_router;
use crate::migrator::Migrator;
use crate::state::ServerState;
use async_graphql::http::{
    playground_source, GraphQLPlaygroundConfig, GraphiQLSource, ALL_WEBSOCKET_PROTOCOLS,
};
use async_graphql::{
    EmptyMutation as GraphEmptyMutation, EmptySubscription as GraphEmptySubscription,
    Schema as GraphSchema,
};
use async_graphql_axum::{GraphQL, GraphQLProtocol, GraphQLWebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::middleware::from_fn;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use axum::{response, Router};
use entities::application_data::Entity as ApplicationData;
use handlers::translation_handler::*;
use query_root::Query as QueryRoot;
use sea_orm::{ConnectOptions, Database, EntityTrait, Set};
use sea_orm_migration::prelude::*;
use serde_json::Value;
use socketioxide::extract::{Bin, Data, SocketRef};
use socketioxide::SocketIo;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

mod entities;
mod handlers;
mod migrator;
mod own_middleware;
mod query_root;
mod state;

fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
    socket.emit("auth", data).ok();

    socket.on(
        "message",
        |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
            info!("Received event: {:?} {:?}", data, bin);
            socket.bin(bin).emit("message-back", data).ok();
        },
    );
}
#[tokio::main]
pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    // Database Setup
    let config_dir = tauri::api::path::config_dir().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Config directory not found")
    })?;
    let folder_path = config_dir.join("translationHero");
    let url = folder_path.join("hero.sqlite?mode=rwc");
    let db_url = format!("sqlite:{}", url.display());
    let mut db_options = ConnectOptions::new(&db_url);
    db_options.sqlx_logging(false);
    println!("{}", db_url);
    let db = Database::connect(db_options)
        .await
        .expect("failed to connect to db");
    Migrator::up(&db, None).await.expect("failed to migrate db");

    match ApplicationData::find_by_id(1).one(&db).await.unwrap() {
        Some(_) => println!("Not first start"),
        None => {
            let data = application_data::ActiveModel {
                theme: Set("Light".to_owned()),
                ..Default::default()
            };
            let _ = ApplicationData::insert(data).exec(&db).await;
        }
    }

    // CORS Setup
    let (layer, io) = SocketIo::new_layer();
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any)
        .allow_credentials(false);

    io.ns("/", on_connect);

    // Graphql
    let schema = GraphSchema::build(QueryRoot, GraphEmptyMutation, GraphEmptySubscription)
        .data(db.clone())
        .finish();

    let state = ServerState {
        db: db,
        schema: schema.clone(),
    };
    let app = Router::new()
        .nest("/store", make_storage_router())
        .nest("/translation", make_translation_router())
        .route("/graphiql", get(graphiql))
        .route("/graphql", get(graphql).post_service(GraphQL::new(schema)))
        // .route("/graphql_ws", get(graphql_ws_handler))
        .layer(cors)
        .layer(from_fn(own_middleware::logger::logger_middleware))
        .layer(layer)
        .with_state(state);

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
async fn graphql() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
async fn graphql_ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<ServerState>,
    protocol: GraphQLProtocol,
) -> Response {
    ws.protocols(ALL_WEBSOCKET_PROTOCOLS)
        .on_upgrade(move |stream| {
            GraphQLWebSocket::new(stream, state.schema.clone(), protocol).serve()
        })
}
