use serde::{Deserialize, Serialize};
use specta::Type;
use std::str::FromStr;
use tauri_specta::Event;
#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
pub struct Data {
    pub settings_store: SettingsStore,
    pub translation_store: TranslationStore,
}
#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
pub struct SettingsStore {
    pub state: SettingsStoreState,
    pub version: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
pub struct SettingsStoreState {
    pub nav_open: bool,
    pub theme: String,
    pub translation_command: String,
    pub run_translation_on_change: bool,
    pub watch_directories: bool,
}

impl FromStr for SettingsStore {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
impl Default for SettingsStore {
    fn default() -> Self {
        Self {
            state: SettingsStoreState {
                nav_open: true,
                theme: "light".to_string(),
                translation_command: "".to_string(),
                run_translation_on_change: false,
                watch_directories: false,
            },
            version: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
pub struct TranslationStore {
    pub state: TranslationStoreState,
    pub version: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
pub struct TranslationStoreState {
    pub test: i32,
}

impl FromStr for TranslationStore {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Default for TranslationStore {
    fn default() -> Self {
        Self {
            state: TranslationStoreState { test: 0 },
            version: 0,
        }
    }
}
