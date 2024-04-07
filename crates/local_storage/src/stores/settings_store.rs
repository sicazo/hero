use crate::types::StoreUpgrade;
use crate::{get_data, StoreType};
use db::prisma::settings;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct SettingsStore {
    pub state: SettingsStoreState,
    pub version: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
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
#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct Notifications {
    #[serde(default)]
    pub file_changes: bool,
    #[serde(default)]
    pub finished_translation: bool,
    #[serde(default)]
    pub finished_scan: bool,
}
#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct ResizablePanelState {
    #[serde(default)]
    pub home_default_sizes: Vec<f32>,
    #[serde(default)]
    pub home_nav_collapsed: bool,
    #[serde(default)]
    pub home_collapsed_size: i32,
}
#[derive(Debug, Serialize, Deserialize, Clone, Type)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
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
            "Light" => Theme::Light,
            "Dark" => Theme::Dark,
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

impl Into<SettingsStore> for settings::Data {
    fn into(self) -> SettingsStore {
        SettingsStore {
            version: 0f32,
            state: SettingsStoreState {
                nav_open: self.nav_open,
                theme: Theme::from(self.theme.as_str()),
                notifications_enabled: self.notifications_enabled,
                toast_rich_colors: self.toast_rich_colors,
                enabled_notification_types: Notifications {
                    file_changes: self.notification_file_changes,
                    finished_translation: self.notification_finished_translation,
                    finished_scan: self.finished_scan,
                },
                translation_settings: TranslationSettings {
                    translate_new_strings: self.translate_new_strings,
                    translate_updated_strings: self.translate_updated_strings,
                    default_language: self.default_language,
                    translation_command: self.translation_command,
                },
                resizable_panel_state: ResizablePanelState {
                    home_default_sizes: vec![
                        self.home_default_size_nav as f32,
                        self.home_default_size_home as f32,
                    ],
                    home_nav_collapsed: self.home_nav_collapsed,
                    home_collapsed_size: self.home_collapsed_nav_size,
                },
            },
        }
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
