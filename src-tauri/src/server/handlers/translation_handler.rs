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
pub struct TranslationHandlerResponse {
    num_of_keys: u32,
}
pub async fn get_number_of_keys(
    Json(payload): Json<TranslationHandlerBody>,
) -> (StatusCode, Json<TranslationHandlerResponse>) {
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
        Json(TranslationHandlerResponse {
            num_of_keys: key_value_len,
        }),
    )
}
