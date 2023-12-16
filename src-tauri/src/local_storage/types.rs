
use serde::{Deserialize, Serialize};
use specta::Type;
use std::error::Error;
use std::str::FromStr;
use tauri_specta::Event;
use crate::stores::settings_store::SettingsStore;
use crate::stores::translation_store::TranslationStore;

pub trait StoreDefault {
    fn default() -> Self;
}

pub trait StoreFromStr {
    type Err;
    fn from_str(s: &str) -> Result<Self, Self::Err>
    where
        Self: std::marker::Sized;
}

pub trait StoreUpgrade {
    fn upgrade(&mut self, current_data_version: f32) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Type, Event)]
pub struct Data {
    pub settings_store: SettingsStore,
    pub translation_store: TranslationStore,
}
