use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Deserialize, Serialize, Type)]
pub struct ScanResponse {
    keys: i32,
    untranslated_keys: i32,
}

#[derive(Deserialize, Serialize, Type)]
pub struct ScanInput {
    pub(crate) path: String,
    pub(crate) name: String,
}

#[derive(Deserialize, Serialize, Type)]
pub struct RescanInput {
    pub(crate) path: String,
    pub(crate) tag: String,
}
