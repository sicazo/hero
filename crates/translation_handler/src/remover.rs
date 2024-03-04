use crate::{PathType, TranslationHandler};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use tracing::error;
impl TranslationHandler {
    pub async fn remove_key(
        path: String,
        ts_keys: Vec<String>,
        json_keys: Vec<String>,
    ) -> Result<(), std::io::Error> {
        let messages_ts_path = PathType::MessageTsFile.create_path(path.clone());
        remove_key_from_messages_ts(messages_ts_path, ts_keys)?;

        Ok(())
    }
}

fn remove_key_from_messages_ts(path: String, keys: Vec<String>) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new().read(true).open(path.clone())?;

    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Failed to read file");

    let mut json_content: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let mut json_content = json_content.as_object_mut().unwrap();
    keys.iter().for_each(|key| {
        if json_content.contains_key(key) {
            json_content.remove(key);
        } else {
            error!(target: "translation_handler", "Could not find key {} in messages.ts in {}", key, path )
        }
    });

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path.clone())?;
    file.write_all(
        serde_json::to_string_pretty(&json_content)
            .unwrap()
            .as_bytes(),
    )?;

    Ok(())
}
