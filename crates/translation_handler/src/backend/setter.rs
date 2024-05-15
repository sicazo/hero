use crate::backend::xml::XmlHandler;
use crate::TranslationHandler;
use db::prisma::settings;
use local_storage::stores::translation_store::TranslationEntry;
use std::collections::HashMap;
use std::path::PathBuf;

impl TranslationHandler {
    pub async fn add_new_backend_key(
        path: String,
        key: String,
        value: String,
        settings: settings::Data,
    ) -> Result<Vec<TranslationEntry>, std::io::Error> {
        XmlHandler::write_key_value(path.clone(), key, value)
            .expect("Failed to add new key_value pair");
        let translations = XmlHandler::read_name_attributes_and_value_tags(path.clone().as_str());
        let translation_entries = translations
            .iter()
            .map(|(key, value)| {
                let values: HashMap<String, String> = value.clone().into_iter().collect();
                TranslationEntry {
                    key: key.to_owned(),
                    value: key.to_owned(),
                    translations: values,
                    in_use: true,
                }
            })
            .collect();
        Ok(translation_entries)
    }
}

fn get_translation_files(path: String) -> Vec<PathBuf> {
    let path = PathBuf::from(path);
    let parent = path.parent().unwrap();
    let mut response: Vec<PathBuf> = Vec::new();
    for entry in parent.read_dir().expect("failed to read parent dir") {
        if let Ok(entry) = entry {
            response.push(entry.path());
        }
    }
    response
}
