use crate::{PathType, TranslationHandler};
use std::fs::OpenOptions;
use std::io::{Read, Write};

impl TranslationHandler {
    pub async fn remove_key(
        path: String,
        ts_key: String,
        json_key: String,
    ) -> Result<(), std::io::Error> {
        let messages_ts_path = PathType::MessageTsFile.create_path(path.clone());
        remove_key_from_messages_ts(messages_ts_path, ts_key.clone())?;

        Ok(())
    }
}

fn remove_key_from_messages_ts(path: String, key: String) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new().read(true).open(path.clone())?;

    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Failed to read file");

    let mut json_content: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    json_content.as_object_mut().unwrap().remove(&key);

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
