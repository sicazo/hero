use crate::local_storage::types::{Data, StoreUpgrade};
use crate::local_storage::{
    get_data, get_settings_file, read_json_file, update_data, write_json_file, StoreType,
};
use crate::stores::settings_store::SettingsStore;
use crate::stores::translation_store::TranslationStore;
use std::io::Read;
use tracing::info;

#[tauri::command]
#[specta::specta]
pub fn remove_store(store: String) {
    let storage = get_settings_file().expect("Failed to open settings.json");
    let mut data: Data = read_json_file("settings.json", &storage).unwrap();
    let store_type = StoreType::from_string(store);
    data = store_type.to_default();
    write_json_file::<Data>(&data).expect("Failed to write to file");
}

#[tauri::command]
#[specta::specta]
pub fn update_store(store: String, value: String) {
    info!("update_store {}", store);
    let store_type = StoreType::from_string(store);
    let data = update_data::<Data>(store_type, value);
    write_json_file::<Data>(&data).expect("Failed to write to file");
}

#[tauri::command]
#[specta::specta]
pub fn get_store(store: String) -> String {
    info!("get_store {}", store);
    let store_type = StoreType::from_string(store);

    get_data::<Data>(store_type)
}

pub fn create_storage() -> Result<(), Box<dyn std::error::Error>> {
    let mut storage = get_settings_file()?;
    let mut contents = String::new();
    storage
        .read_to_string(&mut contents)
        .expect("Failed to read settings.json");
    let mut data: Data = if contents.is_empty() {
        Data {
            settings_store: SettingsStore::default(),
            translation_store: TranslationStore::default(),
        }
    } else {
        serde_json::from_str(&contents)?
    };
    data.upgrade(0.0); // upgrade method is now more sensible
    write_json_file(&data)?;
    Ok(())
}
