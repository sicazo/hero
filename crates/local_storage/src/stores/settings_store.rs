use crate::types::StoreUpgrade;
use crate::{get_data, StoreType};
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
    #[serde(default)]
    pub nav_open: bool,
    pub theme: Theme,
    #[serde(default)]
    pub notifications_enabled: bool,
    #[serde(default)]
    pub toast_rich_colors: bool,
    pub enabled_notification_types: Notifications,
    pub translation_settings: TranslationSettings,
    pub resizable_panel_state: ResizablePanelState,
}
#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
pub struct Notifications {
    #[serde(default)]
    file_changes: bool,
    #[serde(default)]
    finished_translation: bool,
    #[serde(default)]
    finished_scan: bool,
}
#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
pub struct ResizablePanelState {
    #[serde(default)]
    pub home_default_sizes: Vec<f32>,
    #[serde(default)]
    pub home_nav_collapsed: bool,
    #[serde(default)]
    pub home_collapsed_size: i32,
}
#[derive(Debug, Serialize, Deserialize, Clone, Type, Event)]
pub struct TranslationSettings {
    #[serde(default)]
    pub translate_new_strings: bool,
    #[serde(default)]
    pub translate_updated_strings: bool,
    #[serde(default)]
    pub default_language: String,
    #[serde(default)]
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
                toast_rich_colors: true,
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
                resizable_panel_state: {
                    ResizablePanelState {
                        home_default_sizes: vec![265.0, 400.0],
                        home_nav_collapsed: false,
                        home_collapsed_size: 4,
                    }
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
    fn upgrade(&mut self, current_data_version: f32) -> Result<(), Box<dyn Error>> {
        // Upgrade logic for SettingsStore
        if current_data_version == 0.1 {
            self.version = current_data_version;
            self.state.resizable_panel_state = ResizablePanelState {
                home_default_sizes: vec![265.0, 400.0, 655.0],
                home_nav_collapsed: false,
                home_collapsed_size: 4,
            };
        }
        Ok(())
    }
}

impl SettingsStore {
    pub fn get_translation_values() -> TranslationSettings {
        let data = serde_json::from_str::<SettingsStore>(&get_data::<SettingsStore>(
            StoreType::SettingsStoreType,
        ))
        .unwrap();
        data.state.translation_settings
    }
}
