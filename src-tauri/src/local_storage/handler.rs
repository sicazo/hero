use crate::local_storage::types::{Data,  StoreUpgrade};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use crate::stores::settings_store::SettingsStore;
use crate::stores::translation_store::TranslationStore;

#[derive(PartialEq)]
enum StoreType {
    SettingsStore,
    TranslationStore,
}

impl StoreType {
    fn from_string(s: String) -> Self {
        match s.as_str() {
            "settings_store" => StoreType::SettingsStore,
            "translation_store" => StoreType::TranslationStore,
            _ => unreachable!(),
        }
    }
    fn to_default(&self) -> Data {
        match self {
            StoreType::SettingsStore => Data {
                settings_store: SettingsStore::default(),
                ..Data::default()
            },
            StoreType::TranslationStore => Data {
                translation_store: TranslationStore::default(),
                ..Data::default()
            },
        }
    }
}

#[tauri::command]
#[specta::specta]
pub fn remove_store(store: String) {
    let storage = get_settings_file().expect("Failed to open settings.json");
    let mut data: Data = read_json_file("settings.json", &storage).unwrap();
    let store_type = StoreType::from_string(store);
    data = store_type.to_default();
    write_json_file::<Data>(&data).expect("Failed to write to file");
}

fn update_data<T>(store_type: StoreType, value: String) -> Data
where
    T: DeserializeOwned + Default,
{
    let mut storage = get_settings_file().expect("Failed to open settings.json");
    let mut content = String::new();
    storage
        .read_to_string(&mut content)
        .expect("Failed to read settings.json");
    let mut data: Data = serde_json::from_str(&content).unwrap();
    match store_type {
        StoreType::SettingsStore => {
            data.settings_store = serde_json::from_str(&value).unwrap();
        }
        StoreType::TranslationStore => {
            data.translation_store = serde_json::from_str(&value).unwrap();
        }
    };
    data
}
#[tauri::command]
#[specta::specta]
pub fn update_store(store: String, value: String) {
    println!("update_store");
    println!("value: {}", value);
    let store_type = StoreType::from_string(store);
    let data = update_data::<Data>(store_type, value);
    write_json_file::<Data>(&data).expect("Failed to write to file");
}

fn get_data<T>(store_type: StoreType) -> String
where
    T: Serialize + Default,
{
    let storage = get_settings_file().expect("Failed to open settings.json");
    let content: Data = read_json_file("settings.json", &storage).unwrap();
    let result: String;
    match store_type {
        StoreType::SettingsStore => {
            result = serde_json::to_string(&content.settings_store).unwrap();
        }
        StoreType::TranslationStore => {
            result = serde_json::to_string(&content.translation_store).unwrap();
        }
    };
    result
}
#[tauri::command]
#[specta::specta]
pub fn get_store(store: String) -> String {
    println!("get_store");
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

fn read_json_file<T>(_file_path: &str, file: &File) -> Result<T, io::Error>
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
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
}

impl Data {
    pub fn upgrade(&mut self, version: f32) {
        if self.translation_store.version < version {
            self.translation_store
                .upgrade(version)
                .expect("TODO: panic message");
        };
        if self.settings_store.version < version {
            self.settings_store
                .upgrade(version)
                .expect("TODO: panic message");
        };
    }
}
