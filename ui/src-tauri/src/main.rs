// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;

use local_storage::create_storage;
use local_storage::stores::location_store::LocationStoreState;
use local_storage::stores::settings_store::SettingsStoreState;
use local_storage::stores::translation_store::TranslationStoreState;
use local_storage::types::Data;
use sea_orm_migration::MigratorTrait;
use server::get_router;
use server::migrator::MigrationTrait;
use server::state::ServerState;
use std::sync::Arc;
use crate::database::setup_db;

#[tauri::command]
#[specta::specta]
fn greet() -> String {
    "Hello World!".to_string()
}

#[tokio::main]
async fn main() {
    let specta_builder = {
        let specta_builder = tauri_specta::ts::builder()
            .commands(tauri_specta::collect_commands![greet,])
            .events(tauri_specta::collect_events!(
                SettingsStoreState,
                TranslationStoreState,
                LocationStoreState,
                Data
            ));
        let specta_builder = specta_builder.path("../lib/bindings.ts");

        specta_builder.into_plugin()
    };

    let router = get_router();

    // Database Setup
    let db = setup_db().await;

    tauri::Builder::default()
        .plugin(specta_builder)
        .plugin(rspc_tauri::plugin(router.arced(), move |_app_handle| {
            ServerState {
                db: Arc::new(db.clone()),
            }
        }))
        .setup(|_app| {
            create_storage().expect("error while creating storage");
            // thread::spawn(move || init().unwrap());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
