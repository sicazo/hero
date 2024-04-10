// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use local_storage::create_storage;

use db::{context::RouterCtx, load_and_migrate};
use server::{get_router, init};
use std::thread;

#[tokio::main]
async fn main() {
    let router = get_router();

    // Database Setup
    let db = load_and_migrate().await.expect("failed to create db");

    tauri::Builder::default()
        .plugin(rspc_tauri::plugin(router.arced(), move |_| RouterCtx {
            db: db.clone(),
        }))
        .setup(|_app| {
            create_storage().expect("error while creating storage");
            // thread::spawn(move || init().unwrap());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
