use crate::frontend::remover::remove_key_from_language_jsons;
use crate::frontend::setter::{add_translation_to_default_language, run_translation_command};
use crate::{frontend::PathType, TranslationHandler};
use db::prisma::settings;
use glob::glob;
use local_storage::stores::translation_store::TranslationEntry;
use serde::Deserialize;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use tracing::error;

#[derive(Deserialize, Clone, specta::Type, Debug)]
pub struct UpdatedKeyValues {
    pub ts_key: String,
    pub json_key: String,
    pub translation_values: HashMap<String, String>,
}

impl TranslationHandler {
    pub async fn update_keys(
        path: String,
        updated_key: UpdatedKeyValues,
        settings: settings::Data,
    ) -> Result<(), std::io::Error> {
        let translations_directory_path = PathType::TranslationDirectory.create_path(path.clone());
        let json_files = glob(format!("{}/*.json", translations_directory_path).as_str())
            .expect("Failed to read glob pattern");

        // if update is only en-GB and retranslate is enabled, remove all entries of key from other files, just put it in en-GB and run translation
        if updated_key.translation_values.len() == 1
            && updated_key.translation_values.get("en-GB").is_some()
            && settings.translate_updated_strings
        {
            println!("retranslating");
            remove_key_from_language_jsons(
                translations_directory_path,
                vec![updated_key.json_key.clone()],
            )?;
            add_translation_to_default_language(
                path.clone(),
                updated_key.json_key,
                updated_key
                    .translation_values
                    .get("en-GB")
                    .unwrap()
                    .to_owned(),
            )?;
            run_translation_command(path.clone().as_str(), settings.translation_command);
        } else {
            for file in json_files {
                if let Ok(path) = file {
                    let lang = path.file_stem().unwrap().to_str().unwrap();
                    if updated_key.translation_values.contains_key(lang) {
                        update_translation_file(
                            &path,
                            updated_key.json_key.to_owned(),
                            updated_key
                                .translation_values
                                .get(lang)
                                .unwrap()
                                .to_string(),
                        )
                        .await?;
                    }
                } else if let Err(e) = file {
                    error!("{:?}", e);
                }
            }
        }

        Ok(())
    }
}

async fn update_translation_file(
    path: &Path,
    key: String,
    value: String,
) -> Result<(), std::io::Error> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    let mut found = false;

    for line_res in reader.lines() {
        let mut line = line_res?;
        if line.contains(&format!("\"{}\":", &key)) {
            let key_value_regex = regex::Regex::new(r#"(\w+): '(.*)'|"(.*?)": "(.*)""#).unwrap();
            if let Some(capture) = key_value_regex.captures(line.clone().as_str()) {
                let old_value = &capture[4];
                line = line.replace(old_value, value.as_str());
                found = true;
            }
        }
        lines.push(line);
    }

    if !found {
        error!("Key not found")
    } else {
        let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;

        for line in lines {
            writeln!(file, "{}", line)?;
        }
    }
    Ok(())
}
