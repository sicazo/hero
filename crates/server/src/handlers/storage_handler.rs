use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use db::context::RouterCtx;
use db::prisma::PrismaClient;
use db::prisma::{location, settings};
use local_storage::stores::location_store::LocationStore;
use local_storage::stores::settings_store::SettingsStore;
use local_storage::stores::translation_store::TranslationStore;
use local_storage::{get_store, remove_store, update_store};
use rspc::{Router as RspcRouter, RouterBuilder as RspcRouterBuilder};
use serde::{Deserialize, Serialize};
use specta::Type;
use tracing::{error, info};

pub fn make_storage_router() -> Router {
    Router::new()
        .route("/set", post(set_item))
        .route("/get", post(get_item))
        .route("/delete", post(delete_item))
}

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

pub async fn set_item(
    // State(state): State<ServerState>,
    Json(payload): Json<SetStoreBody>,
) -> (StatusCode, Json<()>) {
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

#[derive(Deserialize, Type, Serialize)]
#[serde(untagged)]
enum Store {
    SettingsStore(SettingsStore),
    LocationStore(LocationStore),
    TranslationStore(TranslationStore),
}


pub fn get_storage_router() -> RspcRouterBuilder<RouterCtx> {
    RspcRouter::<RouterCtx>::new()
        .mutation("setStore", |t| {
            t(|ctx, new_settings: Store| async move {
                match new_settings {
                    Store::SettingsStore(store) => {
                        let db: &PrismaClient = &ctx.db;
                        db.settings()
                            .update(
                                settings::id::equals(1),
                                vec![
                                    settings::nav_open::set(store.state.nav_open),
                                    settings::theme::set(format!("{:?}", store.state.theme)),
                                    settings::notifications_enabled::set(
                                        store.state.notifications_enabled,
                                    ),
                                    settings::toast_rich_colors::set(store.state.toast_rich_colors),
                                    settings::notification_file_changes::set(
                                        store.state.enabled_notification_types.file_changes,
                                    ),
                                    settings::notification_finished_translation::set(
                                        store.state.enabled_notification_types.finished_translation,
                                    ),
                                    settings::finished_scan::set(
                                        store.state.enabled_notification_types.finished_scan,
                                    ),
                                    settings::translate_new_strings::set(
                                        store.state.translation_settings.translate_new_strings,
                                    ),
                                    settings::default_language::set(
                                        store.state.translation_settings.default_language,
                                    ),
                                    settings::translation_command::set(
                                        store.state.translation_settings.translation_command,
                                    ),
                                    settings::home_default_size_nav::set(
                                        store.state.resizable_panel_state.home_default_sizes[0]
                                            as i32,
                                    ),
                                    settings::home_default_size_home::set(
                                        store.state.resizable_panel_state.home_default_sizes[1]
                                            as i32,
                                    ),
                                    settings::home_nav_collapsed::set(
                                        store.state.resizable_panel_state.home_nav_collapsed,
                                    ),
                                    settings::home_collapsed_nav_size::set(
                                        store.state.resizable_panel_state.home_collapsed_size,
                                    ),
                                    settings::translate_updated_strings::set(store.state.translation_settings.translate_updated_strings)
                                ],
                            )
                            .exec()
                            .await
                            .expect("failed to update settings");
                    }
                    Store::LocationStore(store) => {
                        println!("Updating locations");
                        // upsert all the state locations

                        for location in &store.state.locations {
                            let db: &PrismaClient = &ctx.db;
                            match db
                                .location()
                                .upsert(
                                    location::UniqueWhereParam::PathEquals(
                                        location.path.to_string(),
                                    ),
                                    (
                                        location.tag.to_string(),
                                        location.name.to_string(),
                                        location.path.to_string(),
                                        location.num_of_keys as i32,
                                        location.num_of_untranslated_keys as i32,
                                        location.added_at.to_string(),
                                        vec![],
                                    ),
                                    vec![
                                        location::num_of_keys::set(location.num_of_keys as i32),
                                        location::num_of_untranslated_keys::set(
                                            location.num_of_untranslated_keys as i32,
                                        ),
                                        location::is_favourite::set(location.is_favourite),
                                    ],
                                )
                                .exec()
                                .await
                            {
                                Ok(_) => {}
                                Err(e) => error!("{e}"),
                            };
                        }

                        // // get all locations
                        let db_locations = &ctx
                            .db
                            .location()
                            .find_many(vec![])
                            .exec()
                            .await
                            .expect(" failed to get locations from db");

                        // // get the one that are in the db but not anymore in state => delete them
                        let mut removed_locs: Vec<String> = Vec::new();
                        for location in &store.state.locations {
                            if db_locations
                                .iter()
                                .any(|db_location| db_location.path == location.path)
                            {
                                info!("loc already in db")
                            } else {
                                removed_locs.push(location.path.clone());
                            }
                        }

                        let futures_vec: Vec<_> = removed_locs
                            .iter()
                            .map(|item| {
                                let item = item.to_owned();

                                let db = ctx.db.clone();
                                tokio::spawn(async move {
                                    match db
                                        .location()
                                        .delete(location::path::equals(item))
                                        .exec()
                                        .await
                                    {
                                        Ok(_) => {}
                                        Err(e) => error!("{:?}", e),
                                    };
                                })
                            })
                            .collect();

                        futures::future::join_all(futures_vec).await;
                    }
                    Store::TranslationStore(store) => {
                        println!("{:?}", store)
                    }
                }
            })
        })
        .mutation("removeStore", |t| {
            t(|ctx, store: String| async move {
                match store.as_str() {
                    "settings_store" => {}
                    "translation_store" => {}
                    "location_store" => {}
                    _ => {}
                }
            })
        })
        .mutation("getStore", |t| {
            t(|ctx, store: String| async move {
                info!("getting store {}", store);
                match store.as_str() {
                    "settings_store" => {
                        let db: &PrismaClient = &ctx.db;
                        let settings = db.settings().find_unique(settings::id::equals(1)).exec().await?.unwrap();
                        Ok(Store::SettingsStore(settings.into()))
                    },
                    "location_store" => {
                        let db: &PrismaClient = &ctx.db;
                        let locations = db.location().find_many(vec![location::name::equals("test".to_string())]).exec().await?;
                        println!("{:?}", locations);
                        Ok(Store::LocationStore(locations.into()))
                    }
                    &_ => todo!()
                }
            })
        })
}
