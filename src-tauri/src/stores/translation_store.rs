use crate::local_storage::types::StoreUpgrade;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::error::Error;
use std::str::FromStr;
use tauri_specta::Event;

#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
pub struct TranslationStore {
    pub state: TranslationStoreState,
    pub version: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
pub struct TranslationStoreState {
    // Example Fields.
    pub test: i32,
}

impl Default for TranslationStore {
    fn default() -> Self {
        Self {
            state: TranslationStoreState { test: 0 },
            version: 0.0,
        }
    }
}

impl FromStr for TranslationStore {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl StoreUpgrade for TranslationStore {
    fn upgrade(&mut self, _current_data_version: f32) -> Result<(), Box<dyn Error>> {
        // Upgrade logic for TranslationStore
        Ok(())
    }
}
