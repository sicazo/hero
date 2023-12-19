use crate::local_storage::types::{Data, StoreUpgrade};
use crate::stores::settings_store::SettingsStore;
use crate::stores::translation_store::TranslationStore;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};
use crate::stores::location_store::LocationStore;

#[derive(PartialEq)]
pub enum StoreType {
    SettingsStore,
    TranslationStore,
    LocationStore
}

impl StoreType {
    pub(crate) fn from_string(s: String) -> Self {
        match s.as_str() {
            "settings_store" => StoreType::SettingsStore,
            "translation_store" => StoreType::TranslationStore,
            "location_store" => StoreType::LocationStore,
            _ => unreachable!(),
        }
    }
    pub(crate) fn to_default(&self) -> Data {
        match self {
            StoreType::SettingsStore => Data {
                settings_store: SettingsStore::default(),
                ..Data::default()
            },
            StoreType::TranslationStore => Data {
                translation_store: TranslationStore::default(),
                ..Data::default()
            },
            StoreType::LocationStore => Data {
                location_store: LocationStore::default(),
                ..Data::default()
            },
        }
    }
}

pub fn update_data<T>(store_type: StoreType, value: String) -> Data
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
        StoreType::LocationStore => {
            data.location_store = serde_json::from_str(&value).unwrap();
        }
    };
    data
}

pub fn get_data<T>(store_type: StoreType) -> String
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
        StoreType::LocationStore => {
            result = serde_json::to_string(&content.location_store).unwrap();
        }
    };
    result
}

pub fn read_json_file<T>(_file_path: &str, file: &File) -> Result<T, io::Error>
where
    T: for<'de> Deserialize<'de>,
{
    let mut file = file.try_clone()?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let data: T = serde_json::from_str(&content)?;
    Ok(data)
}

pub fn write_json_file<T>(data: &T) -> Result<(), io::Error>
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

pub fn get_settings_file() -> Result<File, io::Error> {
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
        if self.location_store.version < version {
            self.settings_store
                .upgrade(version)
                .expect("TODO: panic message");
        };
    }
}
