// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use db::{context::RouterCtx, load_and_migrate};
use server::get_router;

#[tokio::main]
async fn main() {
    let router = get_router();

    // Database Setup
    let db = load_and_migrate().await.expect("failed to create db");

    tauri::Builder::default()
        .plugin(rspc_tauri::plugin(router.arced(), move |_| RouterCtx {
            db: db.clone(),
            location: None
        }))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
