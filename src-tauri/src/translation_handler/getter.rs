use super::TranslationHandler;
use crate::stores::translation_store::TranslationEntry;
use crate::translation_handler::PathType;
use glob::glob;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::fs::read_to_string;
use tracing::info;

impl TranslationHandler {
    fn extract_language_code(line: &str, language_code_regex: &regex::Regex) -> Option<String> {
        let captures = language_code_regex.captures(line)?;
        let language_code = captures.get(1)?;
        Some(language_code.as_str().to_string())
    }

    pub fn extract_language_codes_from_locales(path: String) -> Vec<String> {
        let sub_path = PathType::TranslationExportFile.create_path(path);
        info!("Reading locales.ts file in {}", sub_path);
        let language_code_regex = regex::Regex::new(r"'(\w{2}-\w{2})").unwrap();
        let file_content = read_to_string(&sub_path)
            .expect(format!("Failed to read locales.ts file in {}", sub_path).as_str());
        let file_content = file_content.lines().skip(1);

        let language_codes: Vec<String> = file_content
            .filter_map(|line| Self::extract_language_code(line, &language_code_regex))
            .collect();

        info!("Finished reading locales.ts file in {}", sub_path);
        language_codes
    }

    pub async fn get_key_values_from_messages_ts(path: &str) -> HashMap<String, String> {
        let sub_path = PathType::MessageTsFile.create_path(path.to_owned());
        let file_content = read_to_string(&sub_path)
            .expect(format!("Failed to read messages.ts file in {}", sub_path).as_str());
        let mut mappings = HashMap::new();
        info!("Reading messages.ts file in {}", sub_path);
        let key_value_regex = regex::Regex::new(r#"(\w+): '(.*)'"#).unwrap();
        for capture in key_value_regex.captures_iter(&file_content) {
            let key = capture.get(1).unwrap().as_str().to_string();
            let value = capture.get(2).unwrap().as_str().to_string();
            mappings.insert(key, value);
        }
        mappings
    }
    pub fn read_lang_files_in_locales(
        path: &str,
        keys: HashMap<String, String>,
    ) -> Vec<TranslationEntry> {
        let sub_path = PathType::TranslationDirectory.create_path(path.to_owned());

        let json_files =
            glob(format!("{}/*.json", sub_path).as_str()).expect("Failed to read glob pattern");

        let mut translation_entries: Vec<TranslationEntry> = Vec::new();

        for entry in json_files {
            match entry {
                Ok(path) => {
                    let file_content = fs::read_to_string(&path).expect("Unable to read file");
                    let data: HashMap<String, Value> =
                        serde_json::from_str(&file_content).expect("JSON was not well-formatted");
                    let file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();

                    for (ts_key, json_key) in &keys {
                        if let Some(json_value_object) = data.get(json_key) {
                            let value = json_value_object.as_str().unwrap_or("");
                            if let Some(entry) = translation_entries
                                .iter_mut()
                                .find(|entry| entry.value == *json_key)
                            {
                                entry
                                    .translations
                                    .insert(file_stem.clone(), value.to_string());
                            } else {
                                let mut translations = HashMap::new();
                                translations.insert(file_stem.clone(), value.to_string());
                                let trans_entry = TranslationEntry {
                                    key: ts_key.clone(),
                                    value: json_key.clone(),
                                    translations,
                                    in_use: true,
                                };
                                translation_entries.push(trans_entry);
                            }
                        } else {
                            if let Some(entry) = translation_entries
                                .iter_mut()
                                .find(|entry| entry.value == *json_key)
                            {
                                entry.translations.insert(file_stem.clone(), "".to_owned());
                            } else {
                                let mut translations = HashMap::new();
                                translations.insert(file_stem.clone(), "".to_owned());
                                let trans_entry = TranslationEntry {
                                    key: ts_key.clone(),
                                    value: json_key.clone(),
                                    translations,
                                    in_use: true,
                                };
                                translation_entries.push(trans_entry);
                            }
                        }
                    }
                }
                Err(e) => println!("{:?}", e),
            }
        }
        translation_entries.iter_mut().for_each(|entry| {
            if entry
                .translations
                .iter()
                .all(|(key, value)| value == &"".to_owned())
            {
                entry.in_use = false;
            }
        });
        translation_entries
    }

    pub async fn get_translations(path: &str) -> Vec<TranslationEntry> {
        let keys = Self::get_key_values_from_messages_ts(path).await;
        let translation_entries = Self::read_lang_files_in_locales(path, keys);
        translation_entries
    }
}
