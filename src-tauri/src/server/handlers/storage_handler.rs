use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

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
    crate::local_storage::handler::update_store(payload.name, payload.value);
    (StatusCode::OK, Json(()))
}

pub async fn get_item(Json(payload): Json<GetStoreBody>) -> (StatusCode, Json<String>) {
    let value = crate::local_storage::handler::get_store(payload.name);
    (StatusCode::OK, Json(value))
}

pub async fn delete_item(Json(payload): Json<RemoveStoreBody>) -> (StatusCode, Json<()>) {
    crate::local_storage::handler::remove_store(payload.name);
    (StatusCode::OK, Json(()))
}
