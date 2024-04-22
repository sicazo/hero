pub mod backend;
pub mod frontend;

use db::context::RouterCtx;
use local_storage::stores::translation_store::TranslationEntry;
use serde::{Deserialize, Serialize};
use tracing::info;
use translation_handler::frontend::updater::UpdatedKeyValues;
use translation_handler::TranslationHandler;

use db::prisma;
use db::prisma::{location, settings, PrismaClient};
use frontend::FrontendServerHandler;
use local_storage::stores::location_store::Location;
use rspc::{Router as RspcRouter, RouterBuilder as RspcRouterBuilder};
use db::prisma::settings::Data;

pub struct HandlerData {
    path: String,
    ts_key: String,
    json_key: String,
    en_gb_value: String,
    settings: Data,
}
#[derive(Serialize, specta::Type)]
enum ApiResponse {
    NumberOfKeysResponse(NumberOfKeysResponse),
    TranslationResponse(TranslationsResponse),
}

async fn match_handler(
    action: Actions,
    location: location::Data,
    data: Option<HandlerData>
) -> Result<ApiResponse, rspc::Error> {
    if location.tag == "FE" {
        return Ok(FrontendServerHandler::match_action(action, data).await?);
    } else if location.tag == "BE" {
        // handle backend
        return Ok(FrontendServerHandler::match_action(action, data).await?);
    } else {
        return (Err(rspc::Error::new(
            rspc::ErrorCode::BadRequest,
            "The location is no supported type".to_string(),
        )));
    }
}
enum Actions {
    UpdateKeys,
    GetTranslations,
    GetNumberOfKeys,
    GetLanguages,
    AddKey,
    RemoveKeys,
}
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

#[derive(Serialize, specta::Type)]
pub struct NumberOfKeysResponse {
    num_of_keys: u32,
}

#[derive(Serialize, specta::Type)]
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
        .middleware(|mw| mw.middleware(|mw| async move {
            Ok(mw))
        }))
        .mutation("get_translations", |t| {
            t(|ctx, path: String| async move {
                let db : &PrismaClient = &ctx.db;
                let loc = db
                    .location()
                    .find_unique(location::path::equals(path))
                    .exec()
                    .await?.expect("no location with this path found");
                match_handler(Actions::GetTranslations, loc, None).await
            })
        })
        .query("get_number_of_keys", |t| {
            t(|ctx, path: String| async move {
                let db: &PrismaClient = &ctx.db;
                let loc = db
                    .location()
                    .find_unique(location::path::equals(path))
                    .exec()
                    .await?.expect("no location with this path found");
                match_handler(Actions::GetNumberOfKeys, loc, None).await
            })
        })
        .query("get_languages", |t| {
            t(|ctx, path: String| async move {
                TranslationHandler::extract_language_codes_from_locales(path)
            })
        })
        .mutation("add_key", |t| {
            t(|ctx, input: AddNewKeyBody| async move {
                let db: &PrismaClient = &ctx.db;
                let loc = db
                    .location()
                    .find_unique(location::path::equals(input.path.clone()))
                    .exec()
                    .await?.expect("no location with this path found");
                let settings = db
                    .settings()
                    .find_unique(settings::id::equals(1))
                    .exec()
                    .await?
                    .unwrap();

                match_handler(Actions::AddKey, loc, Some(HandlerData {
                    path: input.path.clone(),
                    ts_key: input.ts_key.clone(),
                    json_key: input.json_key.clone(),
                    en_gb_value: input.value.clone(),
                    settings,
                } )).await

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
