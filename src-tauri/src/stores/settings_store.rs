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
    pub notifications_enabled: bool,
    pub enabled_notification_types: Notifications,
    pub translation_settings: TranslationSettings,
}
#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
pub struct Notifications {
    file_changes: bool,
    finished_translation: bool,
    finished_scan: bool,
}
#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
pub struct TranslationSettings {
    pub translate_new_strings: bool,
    pub translate_updated_strings: bool,
    pub default_language: String,
    pub translation_command: String,
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
                notifications_enabled: false,
                enabled_notification_types: Notifications {
                    file_changes: false,
                    finished_translation: false,
                    finished_scan: false,
                },
                translation_settings: TranslationSettings {
                    translate_new_strings: false,
                    translate_updated_strings: false,
                    default_language: "en-GB".to_string(),
                    translation_command: "".to_string(),
                },
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
