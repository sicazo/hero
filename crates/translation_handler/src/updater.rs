use std::collections::HashMap;
use std::{fs, io};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use glob::glob;
use serde::Deserialize;
use serde_json::{Map, Value};
use tracing::{error, info};
use local_storage::stores::translation_store::TranslationEntry;
use crate::{PathType, TranslationHandler};


#[derive(Deserialize, Clone)]
pub struct UpdatedKeyValues {
    pub ts_key: String,
    pub json_key: String,
    pub translation_values: String,
}


impl TranslationHandler {
    pub async fn update_keys(path: String, updated_key: UpdatedKeyValues) -> Result<(), std::io::Error> {
        let translations_directory_path = PathType::TranslationDirectory.create_path(path.clone());
        let translations = parse_updated_values(updated_key.clone()).await;
        let json_files = glob(format!("{}/*.json", translations_directory_path).as_str()).expect("Failed to read glob pattern");

        for file in json_files {
            if let Ok(path) = file {
                update_translation_file(&path, &updated_key, &translations).await?;
            } else if let Err(e) = file {
                error!("{:?}", e);
            }
        }

        Ok(())
    }


}

async fn parse_updated_values(updated_key_values: UpdatedKeyValues) -> Map<String, Value> {
    let json: serde_json::Value = serde_json::from_str(&updated_key_values.translation_values).expect("failed to parse json");
    json.as_object().expect("failed to create object").to_owned()
}

async fn update_translation_file(path: &Path, updated_key: &UpdatedKeyValues, translations: &Map<String, Value>) -> Result<(), std::io::Error> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    let mut found = false;

    for line_res in reader.lines() {
        let mut line = line_res?;
        if line.contains(&format!("\"{}\":", &updated_key.json_key)) {
            if let Some(new_val) = translations.get(path.file_stem().unwrap().to_str().unwrap()) {
                line = format!("    \"{}\": {}", &updated_key.json_key, new_val);
                found = true;
            }
        }
        lines.push(line);
    }

    if !found {
        error!("Key not found")
    } else {
        // if key was found and replaced, write the updated content back to file
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)?;

        for line in lines {
            writeln!(file, "{}", line)?;
        }
    }
    Ok(())
}

