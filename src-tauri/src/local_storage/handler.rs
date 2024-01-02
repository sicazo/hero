use crate::local_storage::types::{Data, StoreUpgrade};
use crate::local_storage::{get_data, get_settings_file, read_json_file, update_data, write_json_file, StoreType, helper};
use crate::stores::location_store::LocationStore;
use crate::stores::settings_store::SettingsStore;
use crate::stores::translation_store::TranslationStore;
use std::io::Read;
use serde_json::{Value, to_value};
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
    info!("update_store {}", store);
    let store_type = StoreType::from_string(store);
    let data = update_data(store_type, value).expect("Failed to update data");
    write_json_file(&data, store_type).expect("Failed to write to file");
}


pub fn get_store(store: String) -> String {
    info!("get_store {}", store);
    let store_type = StoreType::from_string(store);
    match store_type {
        StoreType::SettingsStore => get_data::<SettingsStore>(store_type),
        StoreType::TranslationStore => get_data::<TranslationStore>(store_type),
        StoreType::LocationStore => get_data::<LocationStore>(store_type)
    }

}

pub fn create_storage() -> Result<(), Box<dyn std::error::Error>> {
    let stores = vec![
        StoreType::SettingsStore,
        StoreType::TranslationStore,
        StoreType::LocationStore,
    ];
    for store in stores {
        let mut storage = get_settings_file(store)?;
        let mut contents = String::new();
        storage
            .read_to_string(&mut contents)
            .expect("Failed to read settings.json");
        let data: Value = if contents.is_empty() {
           match store {
                StoreType::SettingsStore => to_value(&SettingsStore::default()).unwrap(),
                StoreType::TranslationStore => to_value(&TranslationStore::default()).unwrap(),
                StoreType::LocationStore => to_value(&LocationStore::default()).unwrap(),
            }
        } else {
            match store {
                StoreType::SettingsStore => {
                    let mut content: SettingsStore = serde_json::from_str(&contents)?;
                    if content.version < CURRENT_STORE_VERSION {
                         SettingsStore::upgrade(&mut content, CURRENT_STORE_VERSION).expect("Failed to upgrade settings");
                    }
                    to_value(&content).unwrap()
                }
                StoreType::TranslationStore => {
                    let mut content: TranslationStore = serde_json::from_str(&contents)?;
                    if content.version < CURRENT_STORE_VERSION {
                        TranslationStore::upgrade(&mut content, CURRENT_STORE_VERSION).expect("Failed to upgrade translation");
                    }
                    to_value(&content).unwrap()
                }
                StoreType::LocationStore => {
                    let mut content: LocationStore = serde_json::from_str(&contents)?;
                    if content.version < CURRENT_STORE_VERSION {
                        LocationStore::upgrade(&mut content, CURRENT_STORE_VERSION).expect("Failed to upgrade location");
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

