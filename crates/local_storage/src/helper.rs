use crate::stores::location_store::LocationStore;
use crate::stores::settings_store::SettingsStore;
use crate::stores::translation_store::TranslationStore;
use crate::types::{Data, StoreUpgrade};
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};
use tauri::api::path;

#[derive(PartialEq, Clone, Copy)]
pub enum StoreType {
    SettingsStoreType,
    TranslationStoreType,
    LocationStoreType,
}

impl StoreType {
    pub(crate) fn from_string(s: String) -> Self {
        match s.as_str() {
            "settings_store" => StoreType::SettingsStoreType,
            "translation_store" => StoreType::TranslationStoreType,
            "location_store" => StoreType::LocationStoreType,
            _ => unreachable!(),
        }
    }
    pub(crate) fn to_default<T: Default>(&self) -> T {
        match self {
            StoreType::SettingsStoreType => T::default(),
            StoreType::TranslationStoreType => T::default(),
            StoreType::LocationStoreType => T::default(),
        }
    }
}

pub fn update_data(store_type: StoreType, value: String) -> Result<Value, Box<dyn Error>> {
    let mut storage = get_settings_file(store_type)?;
    let mut content = String::new();
    storage.read_to_string(&mut content)?;
    let new_data: Value = serde_json::from_str(&value)?;

    Ok(new_data)
}

pub fn get_data<T>(store_type: StoreType) -> String
where
    T: Serialize + Default,
{
    let storage = get_settings_file(store_type).expect("Failed to open settings.json");
    let content = match store_type {
        StoreType::SettingsStoreType => {
            to_value(read_json_file::<SettingsStore>(&storage).unwrap())
                .expect("Failed to convert to value")
        }
        StoreType::TranslationStoreType => {
            to_value(read_json_file::<TranslationStore>(&storage).unwrap())
                .expect("Failed to convert to value")
        }
        StoreType::LocationStoreType => {
            to_value(read_json_file::<LocationStore>(&storage).unwrap())
                .expect("Failed to convert to value")
        }
    };
    let result = serde_json::to_string(&content).unwrap();
    result
}

pub fn read_json_file<T>(file: &File) -> Result<T, io::Error>
where
    T: for<'de> Deserialize<'de>,
{
    let mut file = file.try_clone()?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let data: T = serde_json::from_str(&content)?;
    Ok(data)
}

pub fn write_json_file<T>(data: &T, store: StoreType) -> Result<(), io::Error>
where
    T: Serialize,
{
    let config_dir = dirs::config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Config directory not found"))?;
    let folder_path = config_dir.join("translationHero");
    let file_path = match store {
        StoreType::SettingsStoreType => folder_path.join("settings.json"),
        StoreType::TranslationStoreType => folder_path.join("translations.json"),
        StoreType::LocationStoreType => folder_path.join("locations.json"),
    };
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;
    let content = serde_json::to_string_pretty(data)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn get_settings_file(store: StoreType) -> Result<File, io::Error> {
    let config_dir = path::config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Config directory not found"))?;
    let folder_path = config_dir.join("translationHero");
    let file_path = match store {
        StoreType::SettingsStoreType => folder_path.join("settings.json"),
        StoreType::TranslationStoreType => folder_path.join("translations.json"),
        StoreType::LocationStoreType => folder_path.join("locations.json"),
    };
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
        if self.location_store.version < version {
            self.settings_store
                .upgrade(version)
                .expect("TODO: panic message");
        };
    }
}
