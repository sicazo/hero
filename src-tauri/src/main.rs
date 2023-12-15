// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod server;
use std::thread;
use tauri::{AppHandle, generate_handler};
mod local_storage;
use local_storage::{remove_store, get_store, update_store, create_storage, HeroStoreState};

#[tauri::command]
#[specta::specta]
fn greet() -> String {
    "Hello World!".to_string()
}



fn main() {

    let specta_builder = {
        let specta_builder = tauri_specta::ts::builder().commands(tauri_specta::collect_commands![greet, remove_store, get_store, update_store]).events(tauri_specta::collect_events!(HeroStoreState));
        #[cfg(debug_assertions)]
        let specta_builder = specta_builder.path("../lib/bindings.ts");

        specta_builder.into_plugin()
    };
    tauri::Builder::default()
    .plugin(specta_builder)
        .setup(|app| {
            create_storage().expect("error while creating storage");
            let handle: AppHandle = app.handle().to_owned();
            let boxed_handle = Box::new(handle);
            thread::spawn(move || server::init(*boxed_handle).unwrap());
            Ok(())
        })
        // .invoke_handler(generate_handler![greet, remove_store, get_store, update_store])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
