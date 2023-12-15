// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod server;
use std::thread;
use tauri::AppHandle;

fn main() {
    let devtools = devtools::init();
    tauri::Builder::default()
        .plugin(devtools)
        .plugin(tauri_plugin_persisted_scope::init())
        .setup(|app| {
            let handle: AppHandle = app.handle().to_owned();
            let boxed_handle = Box::new(handle);
            thread::spawn(move || server::init(*boxed_handle).unwrap());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
