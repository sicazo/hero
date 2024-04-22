pub mod backend;
pub mod frontend;

use db::context::RouterCtx;
use local_storage::stores::translation_store::TranslationEntry;
use serde::{Deserialize, Serialize};
use tracing::info;
use translation_handler::frontend::updater::UpdatedKeyValues;
use translation_handler::TranslationHandler;

use db::prisma::{settings, PrismaClient};
use rspc::{Router as RspcRouter, RouterBuilder as RspcRouterBuilder};

#[derive(Deserialize)]
pub struct PathBody {
    pub path: String,
}

#[derive(Deserialize, specta::Type)]
pub struct RemoveTranslationBody {
    path: String,
    ts_key: Vec<String>,
    json_key: Vec<String>,
}

#[derive(Serialize)]
pub struct NumberOfKeysResponse {
    num_of_keys: u32,
}

#[derive(Serialize)]
pub struct TranslationsResponse {
    keys: Vec<TranslationEntry>,
}

#[derive(Deserialize, specta::Type)]
pub struct AddNewKeyBody {
    path: String,
    ts_key: String,
    json_key: String,
    value: String,
}

#[derive(Deserialize, specta::Type)]
pub struct UpdateKeysBody {
    path: String,
    key: UpdatedKeyValues,
}

pub fn get_translation_router() -> RspcRouterBuilder<RouterCtx> {
    RspcRouter::<RouterCtx>::new()
        .mutation("get_translations", |t| {
            t(|_ctx, path: String| async move { TranslationHandler::get_translations(&path).await })
        })
        .query("get_number_of_keys", |t| {
            t(|_ctx, path: String| async move {
                TranslationHandler::get_translations(&path).await.len() as u32
            })
        })
        .query("get_languages", |t| {
            t(|_ctx, path: String| async move {
                TranslationHandler::extract_language_codes_from_locales(path)
            })
        })
        .mutation("add_key", |t| {
            t(|ctx, input: AddNewKeyBody| async move {
                let db: &PrismaClient = &ctx.db;
                let settings = db
                    .settings()
                    .find_unique(settings::id::equals(1))
                    .exec()
                    .await?
                    .unwrap();
                let keys = TranslationHandler::add_new_key(
                    input.path.clone(),
                    input.ts_key.clone(),
                    input.json_key.clone(),
                    input.value.clone(),
                    settings.clone(),
                )
                .await
                .map_err(|error| {
                    rspc::Error::new(rspc::ErrorCode::InternalServerError, error.to_string())
                })?;

                Ok(keys)
            })
        })
        .mutation("remove_keys", |t| {
            t(|ctx, input: RemoveTranslationBody| async move {
                TranslationHandler::remove_key(input.path, input.ts_key, input.json_key)
                    .await
                    .map_err(|error| {
                        rspc::Error::new(rspc::ErrorCode::InternalServerError, error.to_string())
                    })
            })
        })
        .mutation("update_keys", |t| {
            t(|ctx, input: UpdateKeysBody| async move {
                TranslationHandler::update_keys(input.path, input.key)
                    .await
                    .map_err(|error| {
                        rspc::Error::new(rspc::ErrorCode::InternalServerError, error.to_string())
                    })
            })
        })
}
