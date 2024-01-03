// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use local_storage::create_storage;
use std::thread;
use stores::location_store::LocationStoreState;
use stores::settings_store::SettingsStoreState;
use stores::translation_store::TranslationStoreState;
use crate::local_storage::types::Data;

mod local_storage;
mod server;
pub mod stores;
mod translation_handler;

#[tauri::command]
#[specta::specta]
fn greet() -> String {
    "Hello World!".to_string()
}

fn main() {
    let specta_builder = {
        let specta_builder = tauri_specta::ts::builder()
            .commands(tauri_specta::collect_commands![
                greet,
            ])
            .events(tauri_specta::collect_events!(
                SettingsStoreState,
                TranslationStoreState,
                LocationStoreState,
                Data
            ));
        let specta_builder = specta_builder.path("../lib/bindings.ts");

        specta_builder.into_plugin()
    };
    tauri::Builder::default()
        .plugin(specta_builder)
        .setup(|app| {
            create_storage().expect("error while creating storage");
            thread::spawn(move || server::init().unwrap());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
