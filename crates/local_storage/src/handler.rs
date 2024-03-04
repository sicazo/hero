use crate::stores::location_store::LocationStore;
use crate::stores::settings_store::SettingsStore;
use crate::stores::translation_store::TranslationStore;
use crate::types::{Data, StoreUpgrade};
use crate::{get_data, get_settings_file, read_json_file, update_data, write_json_file, StoreType};
use serde_json::{to_value, Value};
use std::io::Read;
use tracing::info;

const CURRENT_STORE_VERSION: f32 = 1.0;

pub fn remove_store(store: String) {
    let store = StoreType::from_string(store);
    let storage = get_settings_file(store).expect("Failed to open settings.json");
    let mut data: Data = read_json_file(&storage).unwrap();
    data = store.to_default();
    write_json_file::<Data>(&data, store).expect("Failed to write to file");
}

pub fn update_store(store: String, value: String) {
    info!(target: "local_storage","update_store {}", store);
    let store_type = StoreType::from_string(store);
    let data = update_data(store_type, value).expect("Failed to update data");
    write_json_file(&data, store_type).expect("Failed to write to file");
}

pub fn get_store(store: String) -> String {
    info!(target: "local_storage","get_store {}", store);
    let store_type = StoreType::from_string(store);
    match store_type {
        StoreType::SettingsStoreType => get_data::<SettingsStore>(store_type),
        StoreType::TranslationStoreType => get_data::<TranslationStore>(store_type),
        StoreType::LocationStoreType => get_data::<LocationStore>(store_type),
    }
}

pub fn create_storage() -> Result<(), Box<dyn std::error::Error>> {
    let stores = vec![
        StoreType::SettingsStoreType,
        StoreType::TranslationStoreType,
        StoreType::LocationStoreType,
    ];
    for store in stores {
        let mut storage = get_settings_file(store)?;
        let mut contents = String::new();
        storage
            .read_to_string(&mut contents)
            .expect("Failed to read settings.json");
        let data: Value = if contents.is_empty() {
            match store {
                StoreType::SettingsStoreType => to_value(&SettingsStore::default()).unwrap(),
                StoreType::TranslationStoreType => to_value(&TranslationStore::default()).unwrap(),
                StoreType::LocationStoreType => to_value(&LocationStore::default()).unwrap(),
            }
        } else {
            match store {
                StoreType::SettingsStoreType => {
                    let mut content: SettingsStore = serde_json::from_str(&contents)?;
                    if content.version < CURRENT_STORE_VERSION {
                        SettingsStore::upgrade(&mut content, CURRENT_STORE_VERSION)
                            .expect("Failed to upgrade settings");
                    }
                    to_value(&content).unwrap()
                }
                StoreType::TranslationStoreType => {
                    let mut content: TranslationStore = serde_json::from_str(&contents)?;
                    if content.version < CURRENT_STORE_VERSION {
                        TranslationStore::upgrade(&mut content, CURRENT_STORE_VERSION)
                            .expect("Failed to upgrade translation");
                    }
                    to_value(&content).unwrap()
                }
                StoreType::LocationStoreType => {
                    let mut content: LocationStore = serde_json::from_str(&contents)?;
                    if content.version < CURRENT_STORE_VERSION {
                        LocationStore::upgrade(&mut content, CURRENT_STORE_VERSION)
                            .expect("Failed to upgrade location");
                    }
                    to_value(&content).unwrap()
                }
            }
        };

        write_json_file(&data, store)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_path() {
        create_storage().expect("Failed to create storage");
    }
}
