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
    


    

    

    
}

pub fn remove_key_from_language_jsons(
    locales_path: String,
    keys: Vec<String>,
) -> Result<(), std::io::Error> {
    info!(target: "remover", "Removing keys from language JSON files in {}", locales_path);
    info!(target: "remover", "Requested keys to remove: {:?}", keys);
    let json_files =
        glob(format!("{}/*.json", locales_path).as_str()).expect("Failed to read glob pattern");

    for entry in json_files {
        match entry {
            Ok(path) => {
                let file_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                info!(target: "remover", "Processing file: {}", path.display());

                let file_content = fs::read_to_string(&path)?;

                // Detect original indentation
                let indent = file_content
                    .lines()
                    .find(|line| line.trim().starts_with('"'))
                    .map(|line| {
                        let leading = line
                            .chars()
                            .take_while(|c| c.is_whitespace())
                            .collect::<String>();
                        if leading.contains('\t') {
                            "\t".to_string()
                        } else {
                            leading
                        }
                    })
                    .unwrap_or("  ".to_string());
                info!(target: "remover", "Detected indent for {}: {:?}", path.display(), indent);

                // Parse JSON content
                let mut json: HashMap<String, Value> = serde_json::from_str(&file_content)
                    .map_err(|e| {
                        error!(target: "remover", "Failed to parse JSON in {}: {}", path.display(), e);
                        io::Error::new(io::ErrorKind::InvalidData, format!("Invalid JSON: {}", e))
                    })?;

                // Log original content
                info!(target: "remover", "Original content for {}: {:?}", path.display(), json);

                // Remove specified keys
                let original_len = json.len();
                for key in &keys {
                    json.remove(key);
                }

                // Log if keys were removed
                let new_len = json.len();
                if new_len < original_len {
                    info!(target: "remover", "Removed keys from {}: {:?}", path.display(), keys);
                } else {
                    info!(target: "remover", "No matching keys found in {}", path.display());
                }

                // Serialize with original indentation
                let mut new_content = String::new();
                new_content.push_str("{\n");
                let mut entries: Vec<_> = json.into_iter().collect();
                entries.sort_by(|a, b| a.0.cmp(&b.0)); // Sort to maintain consistent order
                for (i, (key, value)) in entries.iter().enumerate() {
                    let formatted_value = serde_json::to_string(value).map_err(|e| {
                        io::Error::new(io::ErrorKind::Other, format!("Serialization error: {}", e))
                    })?;
                    new_content.push_str(&format!(
                        "{}{}: {}",
                        indent,
                        serde_json::to_string(key).unwrap(),
                        formatted_value
                    ));
                    if i < entries.len() - 1 {
                        new_content.push_str(",");
                    }
                    new_content.push_str("\n");
                }
                new_content.push_str("}");

                // Write back to file
                let mut file = OpenOptions::new().write(true).truncate(true).open(&path)?;
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
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Could not find 'locales' in file",
            )
        })?;

    let key_value_pairs = &content[json_start..json_end];
    let mut new_entries = String::new();

    let key_value_regex = Regex::new(r#"\s*(\w+)\s*:\s*(?:"([^"]*)"|'([^']*)')\s*,?\s*"#).unwrap();
    let newline = if std::env::consts::OS == "windows" {
        "\r\n"
    } else {
        "\n"
    };

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

    let new_file = format!(
        "{}{}{}",
        &content[..json_start],
        new_entries,
        &content[json_end..]
    );

    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;
    file.write_all(new_file.as_bytes())?;
    file.flush()?;

    info!(target: "remover", "Successfully removed keys and updated file");
    Ok(())
}
