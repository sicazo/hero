use std::collections::HashMap;
use std::fs;
use crate::{PathType, TranslationHandler};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use glob::glob;
use serde_json::Value;
use tracing::{error, info};
use local_storage::stores::translation_store::TranslationEntry;

impl TranslationHandler {
    pub async fn remove_key(
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

fn remove_key_from_language_jsons(locales_path: String, keys: Vec<String>) -> Result<(), std::io::Error> {
    info!(target: "remover", "Removing keys from language jsons");
    let json_files =
        glob(format!("{}/*.json", locales_path).as_str()).expect("Failed to read glob pattern");
    let newline = if std::env::consts::OS == "windows" {
        "\r\n"
    } else {
        "\n"
    };

    for entry in json_files {
        match entry {
            Ok(path) => {
                let file_content = fs::read_to_string(&path).expect("Unable to read file");
                let first_line = format!("{{{}", newline);
                let mut new_content: String = String::from(first_line);
                let regex = regex::Regex::new(r#""([^"]*)": "([^"]*)""#).expect("failed to create regex");
                file_content.lines().for_each(|line| {
                    if let Some(capture) = regex.captures(line) {
                       if  !keys.clone().into_iter().any(|key| key == capture.get(1).unwrap().as_str() ) {
                           new_content.push_str(&*(line.to_owned() + newline))
                       }
                    }
                });
                new_content.push_str("}");
                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(path.clone())?;
                file.write_all(new_content.as_bytes())?;
                info!(target: "remover", "Removed keys from: {}" , path.display());

            }
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(())
}

fn remove_key_from_messages_ts(path: String, keys: Vec<String>) -> Result<(), std::io::Error> {
    info!(target: "remover", "Removing keys from message.ts");
    let mut file = OpenOptions::new().read(true).open(path.clone())?;

    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Failed to read file");

    let json_start = content.find("export default defineLocales(").unwrap() + "export default defineLocales(".len();
    let json_end = content.find("locales,").unwrap();
    let key_value_pairs = &content[json_start..json_end];
    let mut new_entries: String = String::from("");

    let newline = if std::env::consts::OS == "windows" {
        "\r\n"
    } else {
        "\n"
    };

    key_value_pairs.lines().for_each(|line| {
        let key_value_regex = regex::Regex::new(r#"(\w+): '(.*)'|"(\w+): "(.*)""#).unwrap();
        match key_value_regex.captures(line) {
            None => new_entries.push_str(&*(line.to_owned() + newline)),
            Some(capture) => {
                if !keys.iter().any(|key| &capture.get(1).unwrap().as_str().to_string() == key) {
                    new_entries.push_str(&*(line.to_owned() + newline));
                }
            }
        }
    });

    if new_entries.ends_with(newline) {
        new_entries.pop();
        if newline.len() > 1 {
            new_entries.pop();
        }
    }

    let new_file = (&content[0..json_start]).to_owned() + new_entries.as_str() + &content[json_end..];

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path.clone())?;

    file.write_all(new_file.as_bytes())?;

    Ok(())
}