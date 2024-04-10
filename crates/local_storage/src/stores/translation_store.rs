use crate::types::StoreUpgrade;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

const DEFAULT_LANGUAGES: [&str; 34] = [
    "de-DE", "de-AT", "de-CH", "de-LU", "nl-NL", "nl-BE", "en-GB", "en-US", "es-ES", "fr-FR",
    "fr-BE", "fr-CH", "it-IT", "it-CH", "pl-PL", "pt-PT", "hu-HU", "hr-HR", "sr-La", "sl-SI",
    "el-GR", "bg-BG", "ro-RO", "tr-TR", "da-DK", "fi-FI", "nb-NO", "sv-SE", "sk-SK", "cs-CZ",
    "uk-UA", "et-EE", "lt-LT", "lv-LV",
];

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct TranslationStore {
    pub state: TranslationStoreState,
    pub version: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct TranslationStoreState {
    #[serde(default)]
    pub languages: Vec<String>,
    pub translation_entries: Vec<TranslationEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct TranslationEntry {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub translations: HashMap<String, String>,
    #[serde(default)]
    pub in_use: bool,
}

impl TranslationEntry {
    fn new(key: &str, value: &str, translations: HashMap<String, String>, in_use: bool) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
            translations,
            in_use,
        }
    }
}

impl Default for TranslationStore {
    fn default() -> Self {
        Self {
            state: TranslationStoreState {
                languages: DEFAULT_LANGUAGES.iter().map(|s| s.to_string()).collect(),
                translation_entries: vec![],
            },
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
    fn upgrade(&mut self, current_data_version: f32) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
