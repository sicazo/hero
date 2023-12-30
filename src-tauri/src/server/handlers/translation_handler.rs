use crate::stores::translation_store::TranslationEntry;
use crate::translation_handler::TranslationHandler;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::info;

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
