use crate::backend::xml::XmlReader;
use crate::TranslationHandler;
use glob::glob;
use local_storage::stores::translation_store::TranslationEntry;
use std::collections::{BTreeMap, HashMap};
use std::fs::read_to_string;
use std::path::Path;

pub fn get_translations_from_location(
    location_path: &str,
) -> BTreeMap<String, BTreeMap<String, String>> {
    let xml = read_to_string(location_path).expect("failed to read file");
    let response = XmlReader::read_name_attributes_and_value_tags(&xml);
    response
}

pub fn get_resources_from_csproj(path: &str) -> Option<Vec<String>> {
    if !path.ends_with(".csproj") {
        return None;
    }
    let xml = read_to_string(path).expect("failed to read file");
    let mut response = XmlReader::get_resources(&xml, path);
    response.retain(|file| {
        let path = std::path::Path::new(file);
        let filename = path
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        path.extension().map_or(false, |ext| ext == "resx")
            && !filename.contains('.')
            && path.exists()
    });
    Some(response)
}

impl TranslationHandler {
    pub async fn get_backend_translations(path: &str) -> Vec<TranslationEntry> {
        let keys = get_translations_from_location(path);
        let path = Path::new(path);
        let parent_folder = path.parent().unwrap();
        let translations = Self::get_locale_translations(
            parent_folder.to_str().unwrap().to_string(),
            path.file_stem().unwrap().to_str().unwrap().to_string(),
            keys.clone(),
        )
        .await;
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
        translation_entries
    }

    async fn get_locale_translations(
        parent_folder: String,
        main_file: String,
        keys: BTreeMap<String, BTreeMap<String, String>>,
    ) -> BTreeMap<String, BTreeMap<String, String>> {
        let main_file = format!("{}.resx", main_file);
        let resx_files = glob(format!("{}/*.resx", parent_folder).as_str())
            .expect("Failed to read glob pattern");
        let mut keys = keys.clone();

        for entry in resx_files {
            match entry {
                Ok(path) => {
                    if let Some(filename) = path.file_stem() {
                        if let Some(file_name_str) = filename.to_str() {
                            if !file_name_str.ends_with(&main_file) {
                                let language_code: Vec<&str> = file_name_str.split(".").collect();
                                let file_translation_entries =
                                    get_translations_from_location(path.clone().to_str().unwrap());
                                if language_code.len() > 1 {
                                    file_translation_entries.iter().for_each(|(key, value)| {
                                        if let Some(inner_map) = keys.get_mut(key) {
                                            inner_map.insert(
                                                language_code[1].to_string(),
                                                value.get("default").unwrap().to_owned(),
                                            );
                                        }
                                    })
                                }
                            }
                        }
                    }
                }
                Err(_) => {
                    // You can decide to do something in case of an error here (for example, log the error)
                }
            }
        }

        keys
    }
}
