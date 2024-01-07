use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use local_storage::{get_store, remove_store, update_store};

#[derive(Deserialize)]
pub struct SetStoreBody {
    name: String,
    value: String,
}
#[derive(Deserialize)]
pub struct GetStoreBody {
    name: String,
}
#[derive(Deserialize)]
pub struct RemoveStoreBody {
    name: String,
}

pub async fn set_item(Json(payload): Json<SetStoreBody>) -> (StatusCode, Json<()>) {
    update_store(payload.name, payload.value);
    (StatusCode::OK, Json(()))
}

pub async fn get_item(Json(payload): Json<GetStoreBody>) -> (StatusCode, Json<String>) {
    let value = get_store(payload.name);
    (StatusCode::OK, Json(value))
}

pub async fn delete_item(Json(payload): Json<RemoveStoreBody>) -> (StatusCode, Json<()>) {
    remove_store(payload.name);
    (StatusCode::OK, Json(()))
}
