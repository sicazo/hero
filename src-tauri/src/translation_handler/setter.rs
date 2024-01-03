use crate::stores::settings_store::SettingsStore;
use crate::translation_handler::{PathType, TranslationHandler};
use std::fs::OpenOptions;
use std::io::{Read, Write};

impl TranslationHandler {
    pub fn add_new_key(
        path: String,
        ts_key: String,
        json_key: String,
        en_gb_value: String,
    ) -> Result<(), std::io::Error> {
        let settings = SettingsStore::get_translation_values();
        let messages_ts_path = PathType::MessageTsFile.create_path(path.clone());
        let locales_path = PathType::TranslationDirectory.create_path(path.clone());
        let mut file = OpenOptions::new()
            .read(true)
            .open(messages_ts_path.clone())?;

        let mut lines: Vec<String> = Vec::new();
        let mut check_line = 0;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read file");
        content.lines().enumerate().for_each(|(index, line)| {
            if line.contains("},") {
                check_line = index - 1;
                lines.push(String::from("    myKey: 'myValue',"));
            }
            lines.push(String::from(line))
        });
        lines[check_line] = if !lines[check_line].ends_with(',') {
            format!("{},", lines[check_line])
        } else {
            lines[check_line].clone()
        };

        let lines_content: String = lines.join("\n");
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(messages_ts_path.clone())?;
        file.write_all(lines_content.as_bytes())?;

        // add to en-gb.json now and run translation command afterwards

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_new_key() {
        let path = String::from("/Users/marius/Developer/Github/translation_hero/testfiles");
        let ts_key = String::from("test");
        let json_key = String::from("test");
        let en_gb_value = String::from("test");
        let result = TranslationHandler::add_new_key(path, ts_key, json_key, en_gb_value);
    }
}
