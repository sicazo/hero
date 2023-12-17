use crate::local_storage::types::StoreUpgrade;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;
use tauri_specta::Event;

#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
pub struct SettingsStore {
    pub state: SettingsStoreState,
    pub version: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
pub struct SettingsStoreState {
    pub nav_open: bool,
    pub theme: Theme,
    pub translation_command: String,
    pub run_translation_on_change: bool,
    pub watch_directories: bool,
}
#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
}
impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Theme::Light => "light".to_string(),
            Theme::Dark => "dark".to_string(),
        };
        write!(f, "{}", str)
    }
}
impl From<&str> for Theme {
    fn from(s: &str) -> Self {
        match s {
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            _ => unreachable!(),
        }
    }
}
impl Default for Theme {
    fn default() -> Self {
        Theme::Light
    }
}
impl Default for SettingsStore {
    fn default() -> Self {
        Self {
            state: SettingsStoreState {
                nav_open: true,
                theme: Theme::default(),
                translation_command: "".to_string(),
                run_translation_on_change: false,
                watch_directories: false,
            },
            version: 0.0,
        }
    }
}

impl FromStr for SettingsStore {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl StoreUpgrade for SettingsStore {
    fn upgrade(&mut self, _current_data_version: f32) -> Result<(), Box<dyn Error>> {
        // Upgrade logic for SettingsStore
        Ok(())
    }
}
