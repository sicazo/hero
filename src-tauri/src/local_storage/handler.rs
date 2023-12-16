use crate::local_storage::types::{Data, SettingsStore, TranslationStore};
use crate::local_storage::{SettingsStoreState, TranslationStoreState};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};

#[tauri::command]
#[specta::specta]
pub fn remove_store(store: String) {
    let storage = get_settings_file().expect("Failed to open settings.json");

    let mut data: Data = read_json_file("settings.json", &storage).unwrap();

    match store.as_str() {
        "settings_store" => {
            data.settings_store = SettingsStore::default();
        }
        "translation_store" => {
            data.translation_store = TranslationStore::default();
        }
        _ => unreachable!(),
    };
    write_json_file::<Data>(&data.into()).expect("Failed to write to file");
}

#[tauri::command]
#[specta::specta]
pub fn get_store(store: String) -> String {
    println!("get_store");
    let storage = get_settings_file().expect("Failed to open settings.json");

    let content: Data = read_json_file("settings.json", &storage).unwrap();
    let json = match store.as_str() {
        "settings_store" => serde_json::to_string(&content.settings_store).unwrap(),
        "translation_store" => serde_json::to_string(&content.translation_store).unwrap(),
        _ => unreachable!(),
    };

    json
}

#[tauri::command]
#[specta::specta]
pub fn update_store(store: String, value: String) {
    println!("update_store");
    println!("value: {}", value);
    let mut storage = get_settings_file().expect("Failed to open settings.json");

    let mut content = String::new();
    storage
        .read_to_string(&mut content)
        .expect("Failed to read settings.json");

    let mut data: Data = serde_json::from_str(&content).unwrap();
    match store.as_str() {
        "settings_store" => {
            data.settings_store = value.parse::<SettingsStore>().unwrap();
        }
        "translation_store" => {
            data.translation_store = value.parse::<TranslationStore>().unwrap();
        }
        _ => unreachable!(),
    };

    write_json_file::<Data>(&data.into()).expect("Failed to write to file");
}

pub fn create_storage() -> Result<(), io::Error> {
    let mut storage = get_settings_file()?;

    let mut contents = String::new();
    storage
        .read_to_string(&mut contents)
        .expect("Failed to read settings.json");

    if contents.is_empty() {
        let default_data = Data {
            settings_store: SettingsStore::default(),
            translation_store: TranslationStore::default(),
        };
        write_json_file(&default_data)?;
    }

    Ok(())
}

fn read_json_file<T>(file_path: &str, file: &File) -> Result<T, io::Error>
where
    T: for<'de> Deserialize<'de>,
{
    let mut file = file.try_clone()?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let data: T = serde_json::from_str(&content)?;

    Ok(data)
}

fn write_json_file<T>(data: &T) -> Result<(), io::Error>
where
    T: Serialize,
{
    let config_dir = dirs::config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Config directory not found"))?;
    let folder_path = config_dir.join("translationHero");
    let file_path = folder_path.join("settings.json");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    let content = serde_json::to_string_pretty(data)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}

fn get_settings_file() -> Result<File, io::Error> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Config directory not found"))?;
    let folder_path = config_dir.join("translationHero");
    let file_path = folder_path.join("settings.json");

    if !folder_path.exists() {
        std::fs::create_dir_all(&folder_path)?;
    }

    Ok(OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?)
}
