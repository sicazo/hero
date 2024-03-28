use crate::types::StoreUpgrade;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::error::Error;
use std::str::FromStr;
use tauri_specta::Event;

#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
pub struct LocationStore {
    pub state: LocationStoreState,
    pub version: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
pub struct LocationStoreState {
    #[serde(default)]
    pub last_selected_location: Option<Location>,
    #[serde(default)]
    pub locations: Vec<Location>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
pub struct Location {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub is_favourite: bool,
    #[serde(default)]
    pub num_of_keys: u32,
    #[serde(default)]
    pub num_of_untranslated_keys: u32,
    #[serde(default)]
    pub added_at: String,
}

impl Default for LocationStore {
    fn default() -> Self {
        Self {
            state: LocationStoreState {
                last_selected_location: None,
                locations: vec![],
            },
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
    fn upgrade(&mut self, current_data_version: f32) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
