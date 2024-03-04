use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use local_storage::stores::translation_store::TranslationEntry;
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use translation_handler::TranslationHandler;

pub fn translation_router() -> Router {
    Router::new()
        .route("/keys", post(get_number_of_keys))
        .route("/translations", post(get_translations))
        .route("/add", post(add_new_key))
        .route("/languages", post(get_languages))
        .route("/remove", post(remove_keys))
}
#[derive(Deserialize)]
pub struct TranslationHandlerBody {
    path: String,
}

#[derive(Deserialize)]
pub struct RemoveTranslationBody {
    path: String,
    ts_key: Vec<String>,
    json_key: Vec<String>
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

#[derive(Deserialize)]
pub struct GetLanguagesBody {
    path: String,
}


pub async fn remove_keys(Json(payload): Json<RemoveTranslationBody>) -> StatusCode {
    info!(target: "server_action", "Removing keys from {}",&payload.path);
    match TranslationHandler::remove_key(payload.path, payload.ts_key, payload.json_key).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    }


}

pub async fn get_number_of_keys(
    Json(payload): Json<TranslationHandlerBody>,
) -> (StatusCode, Json<NumberOfKeysResponse>) {
    info!(target: "server_action", "Getting number of keys for {}", &payload.path);
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
    info!("Adding new key {} to {}", &payload.ts_key, &payload.path);
    match TranslationHandler::add_new_key(
        payload.path.clone(),
        payload.ts_key.clone(),
        payload.json_key.clone(),
        payload.value.clone(),
    )
    .await
    {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn get_languages(
    Json(payload): Json<GetLanguagesBody>,
) -> (StatusCode, Json<Vec<String>>) {
    info!("Getting language codes from {}", &payload.path);
    let languages = TranslationHandler::extract_language_codes_from_locales(payload.path);
    (StatusCode::OK, Json(languages))
}
