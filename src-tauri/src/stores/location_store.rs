use crate::local_storage::types::StoreUpgrade;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;
use tauri_specta::Event;

#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
pub struct LocationStore {
    pub state: LocationStoreState,
    pub version: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
pub struct LocationStoreState {
    pub locations: Vec<Location>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
pub struct Location {
    pub name: String,
    pub is_favourite: bool,
    pub num_of_keys: u32,
    pub num_of_untranslated_keys: u32,
    pub added_at: String,
}

impl Default for LocationStore {
    fn default() -> Self {
        Self {
            state: LocationStoreState { locations: vec![] },
            version: 0.0,
        }
    }
}

impl FromStr for LocationStore {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl StoreUpgrade for LocationStore {
    fn upgrade(&mut self, _current_data_version: f32) -> Result<(), Box<dyn Error>> {
        // Upgrade logic for SettingsStore
        Ok(())
    }
}
