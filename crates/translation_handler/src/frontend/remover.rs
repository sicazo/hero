use crate::{frontend::PathType, TranslationHandler};
use glob::glob;
use local_storage::stores::translation_store::TranslationEntry;
use regex::Regex;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::fs::{self, read_to_string, OpenOptions};
use std::io::{self, Read, Write};
use tracing::{error, info};

impl TranslationHandler {
    pub fn extract_language_code(line: &str, language_code_regex: &Regex) -> Option<String> {
        let captures = language_code_regex.captures(line)?;
        let language_code = captures.get(1)?;
        Some(language_code.as_str().to_string())
    }

    pub fn extract_language_codes_from_locales(path: String) -> Vec<String> {
        let sub_path = PathType::TranslationExportFile.create_path(path);
        info!("Reading locales.ts file in {}", sub_path);
        let language_code_regex = Regex::new(r"'(\w{2}-\w{2})").unwrap();
        let file_content = read_to_string(&sub_path)
            .expect(&format!("Failed to read locales.ts file in {}", sub_path));
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
            .expect(&format!("Failed to read messages.ts file in {}", sub_path));
        let mut mappings = HashMap::new();
        info!(target: "translation", "Reading messages.ts file in {}", sub_path);
        info!(target: "translation", "File content (first 200 chars): {}", file_content.chars().take(200).collect::<String>());

        
        let re = Regex::new(r"(?s)\s*export\s+default\s+defineLocales\s*\(\s*\{(.*)\}\s*,\s*locales\s*").unwrap();
        let json_content = re
            .captures(&file_content)
            .and_then(|cap| cap.get(1).map(|m| m.as_str()))
            .unwrap_or(&file_content);
        info!(target: "translation", "Extracted JSON content (first 200 chars): {}", json_content.chars().take(200).collect::<String>());

        // Regex to match key: 'value' or key: "value" with optional comma and whitespace
        let key_value_regex = Regex::new(r#"\s*(\w+)\s*:\s*(?:"([^"]*)"|'([^']*)')\s*,?\s*"#).unwrap();
        for capture in key_value_regex.captures_iter(json_content) {
            let key = capture.get(1).unwrap().as_str().to_string();
            let value = capture
                .get(2)
                .or(capture.get(3))
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            mappings.insert(key.clone(), value.clone());
            info!(target: "translation", "Captured key: {}, value: {}", key, value);
        }

        info!(target: "translation", "Total keys captured: {}", mappings.len());
        mappings
    }

    pub fn read_lang_files_in_locales(
        path: &str,
        keys: HashMap<String, String>,
    ) -> Vec<TranslationEntry> {
        let sub_path = PathType::TranslationDirectory.create_path(path.to_owned());

        let json_files = glob(format!("{}/*.json", sub_path).as_str()).expect("Failed to read glob pattern");

        let mut translation_entries: Vec<TranslationEntry> = Vec::new();

        for entry in json_files {
            match entry {
                Ok(path) => {
                    let file_content = fs::read_to_string(&path).expect("Unable to read file");
                    let data: HashMap<String, Value> = serde_json::from_str(&file_content).expect(
                        &format!("JSON was not well-formatted in {}", path.display()),
                    );
                    let file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();

                    for (ts_key, json_key) in &keys {
                        if let Some(json_value_object) = data.get(json_key) {
                            let value = json_value_object.as_str().unwrap_or("");
                            if let Some(entry) = translation_entries
                                .iter_mut()
                                .find(|entry| entry.value == *json_key)
                            {
                                entry.translations.insert(file_stem.clone(), value.to_string());
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
            if entry.translations.iter().all(|(_key, value)| value == &"".to_owned()) {
                entry.in_use = false;
            }
        });
        translation_entries
    }

    pub async fn get_frontend_translations(path: &str) -> Vec<TranslationEntry> {
        let keys = Self::get_key_values_from_messages_ts(path).await;
        let translation_entries = Self::read_lang_files_in_locales(path, keys);
        translation_entries
    }

    pub async fn remove_frontend_key(
        path: String,
        ts_keys: Vec<String>,
        json_keys: Vec<String>,
    ) -> Result<(), std::io::Error> {
        let messages_ts_path = PathType::MessageTsFile.create_path(path.clone());
        remove_key_from_messages_ts(messages_ts_path, ts_keys)?;
        let locales_path = PathType::TranslationDirectory.create_path(path.clone());
        remove_key_from_language_jsons(locales_path, json_keys)?;

        Ok(())
    }
}

pub fn remove_key_from_language_jsons(
    locales_path: String,
    keys: Vec<String>,
) -> Result<(), std::io::Error> {
    info!(target: "remover", "Removing keys from language JSON files in {}", locales_path);
    info!(target: "remover", "Requested keys to remove: {:?}", keys);
    let json_files = glob(format!("{}/*.json", locales_path).as_str()).expect("Failed to read glob pattern");

    for entry in json_files {
        match entry {
            Ok(path) => {
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown");
                info!(target: "remover", "Processing file: {}", path.display());

                let file_content = fs::read_to_string(&path)?;
                
                // Detect original indentation and newline style
                let indent = file_content
                    .lines()
                    .find(|line| line.trim().starts_with('"'))
                    .map(|line| {
                        line.chars().take_while(|c| c.is_whitespace()).collect::<String>()
                    })
                    .unwrap_or("  ".to_string());
                let newline = if file_content.contains("\r\n") { "\r\n" } else { "\n" };
                info!(target: "remover", "Detected indent for {}: {:?}", path.display(), indent);
                info!(target: "remover", "Detected newline for {}: {:?}", path.display(), newline);

                // Parse JSON content while preserving order
                let json: serde_json::Map<String, Value> = serde_json::from_str(&file_content)
                    .map_err(|e| {
                        error!(target: "remover", "Failed to parse JSON in {}: {}", path.display(), e);
                        io::Error::new(io::ErrorKind::InvalidData, format!("Invalid JSON: {}", e))
                    })?;

                // Log original content
                info!(target: "remover", "Original content for {}: {:?}", path.display(), json);

                // Remove specified keys while preserving order
                let mut new_json = serde_json::Map::new();
                for (key, value) in json.clone() {
                    if !keys.contains(&key) {
                        new_json.insert(key, value);
                    }
                }

                // Log if keys were removed
                let original_len = json.len();
                let new_len = new_json.len();
                if new_len < original_len {
                    info!(target: "remover", "Removed keys from {}: {:?}", path.display(), keys);
                } else {
                    info!(target: "remover", "No matching keys found in {}", path.display());
                }

                // Serialize with original formatting
                let mut new_content = String::new();
                new_content.push_str("{");
                new_content.push_str(newline);
                let entries: Vec<_> = new_json.into_iter().collect(); // Preserve order
                for (i, (key, value)) in entries.iter().enumerate() {
                    let formatted_value = serde_json::to_string(value)
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Serialization error: {}", e)))?;
                    new_content.push_str(&format!(
                        "{}\"{}\": {}{}",
                        indent,
                        key,
                        formatted_value,
                        if i < entries.len() - 1 { "," } else { "" }
                    ));
                    new_content.push_str(newline);
                }
                new_content.push_str("}");

                // Write back to file
                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(&path)?;
                file.write_all(new_content.as_bytes())?;
                file.flush()?;
                info!(target: "remover", "Successfully updated: {}", path.display());
            }
            Err(e) => {
                error!(target: "remover", "Error reading file: {:?}", e);
            }
        }
    }
    Ok(())
}

pub fn remove_key_from_messages_ts(path: String, keys: Vec<String>) -> Result<(), std::io::Error> {
    info!(target: "remover", "Removing keys from message.ts at {}", path);
    let mut file = OpenOptions::new().read(true).open(path.clone())?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let re = Regex::new(r"(?s)\s*export\s+default\s+defineLocales\s*\(").unwrap();
    let json_start = re.find(&content).map(|m| m.end()).ok_or_else(|| {
        info!(target: "remover", "Did not find 'export default defineLocales(' in {}", path);
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Could not find 'export default defineLocales(' pattern",
        )
    })?;

    let json_end = content[json_start..]
        .find("locales")
        .map(|i| json_start + i)
        .ok_or_else(|| {
            info!(target: "remover", "Did not find 'locales' in {}", path);
            io::Error::new(io::ErrorKind::InvalidData, "Could not find 'locales' in file")
        })?;

    let key_value_pairs = &content[json_start..json_end];
    let mut new_entries = String::new();

    let key_value_regex = Regex::new(r#"\s*(\w+)\s*:\s*(?:"([^"]*)"|'([^']*)')\s*,?\s*"#).unwrap();
    let newline = if std::env::consts::OS == "windows" { "\r\n" } else { "\n" };

    key_value_pairs
        .lines()
        .for_each(|line| match key_value_regex.captures(line) {
            None => new_entries.push_str(&format!("{}{}", line, newline)),
            Some(capture) => {
                let key = capture.get(1).unwrap().as_str();
                if !keys.iter().any(|k| k == key) {
                    new_entries.push_str(&format!("{}{}", line, newline));
                }
            }
        });

    if new_entries.ends_with(newline) {
        new_entries.pop();
        if newline.len() > 1 {
            new_entries.pop();
        }
    }

    let new_file = format!("{}{}{}", &content[..json_start], new_entries, &content[json_end..]);

    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;
    file.write_all(new_file.as_bytes())?;
    file.flush()?;

    info!(target: "remover", "Successfully removed keys and updated file");
    Ok(())
}