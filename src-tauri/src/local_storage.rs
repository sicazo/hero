use std::{fs::OpenOptions, io::{Read, Write}};



#[tauri::command]
#[specta::specta]
pub fn set_key() -> String {
    "Hello World!".to_string()
}

#[tauri::command]
#[specta::specta]
pub fn get_key() -> String {
    "Hello World!".to_string()
}

#[tauri::command]
#[specta::specta]
pub fn update_key() -> String {
    "Hello World!".to_string()
}

pub fn create_storage() -> Result<(), Box<dyn std::error::Error>> {
    let mut storage = OpenOptions::new().read(true).write(true).create(true).open("settings.json").expect("Failed to create storage.json");
    let mut contents = String::new();
    storage.read_to_string(&mut contents)?;
    if contents == "" {
        storage.write_all(b"{}")?;
    }
    Ok(())
}