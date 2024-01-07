use translation_handler::TranslationHandler;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::info;
use local_storage::stores::translation_store::TranslationEntry;

#[derive(Deserialize)]
pub struct TranslationHandlerBody {
    path: String,
}
#[derive(Serialize)]
pub struct NumberOfKeysResponse {
    num_of_keys: u32,
}
#[derive(Serialize)]
pub struct TranslationsResponse {
    keys: Vec<TranslationEntry>,
}
#[derive(Deserialize)]
pub struct AddNewKeyBody {
    path: String,
    ts_key: String,
    json_key: String,
    value: String,
}
pub async fn get_number_of_keys(
    Json(payload): Json<TranslationHandlerBody>,
) -> (StatusCode, Json<NumberOfKeysResponse>) {
    info!("Getting number of keys for {}", &payload.path);
    let key_value_len = TranslationHandler::get_key_values_from_messages_ts(&payload.path)
        .await
        .len() as u32;
    info!(
        "Responding with {} keys for {}",
        &key_value_len, &payload.path
    );
    (
        StatusCode::OK,
        Json(NumberOfKeysResponse {
            num_of_keys: key_value_len,
        }),
    )
}
pub async fn get_translations(
    Json(payload): Json<TranslationHandlerBody>,
) -> (StatusCode, Json<TranslationsResponse>) {
    info!("Getting keys from {}", &payload.path);
    let key_value = TranslationHandler::get_translations(&payload.path).await;
    info!("{}", key_value.len());
    (
        StatusCode::OK,
        Json(TranslationsResponse { keys: key_value }),
    )
}

pub async fn add_new_key(Json(payload): Json<AddNewKeyBody>) -> StatusCode {
    info!("Adding new key {} to {}",&payload.ts_key, &payload.path);
    match TranslationHandler::add_new_key(
        payload.path.clone(),
        payload.ts_key.clone(),
        payload.json_key.clone(),
        payload.value.clone(),
    ).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    }

}
