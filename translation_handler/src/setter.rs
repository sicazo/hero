use crate::{PathType, TranslationHandler};
use local_storage::stores::settings_store::SettingsStore;
use serde::Serialize;
use serde_json::ser::PrettyFormatter;
use std::fs::{read_to_string, OpenOptions};
use std::io::{Read, Write};
use std::process::Command;

impl TranslationHandler {
    pub async fn add_new_key(
        path: String,
        ts_key: String,
        json_key: String,
        en_gb_value: String,
    ) -> Result<(), std::io::Error> {
        let settings = SettingsStore::get_translation_values();
        let messages_ts_path = PathType::MessageTsFile.create_path(path.clone());

        add_key_to_messages_ts(messages_ts_path, ts_key.clone(), json_key.clone())?;

        add_translation_to_default_language(path.clone(), json_key, en_gb_value)?;

        if settings.translate_new_strings {
            run_translation_command(&path.clone(), settings.translation_command);
        }

        Ok(())
    }
}
fn run_translation_command(dir_path: &str, translation_command: String) {
    let locales_path = PathType::TranslationDirectory.create_path(dir_path.to_string());
    let program = if cfg!(target_os = "windows") {
        "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe"
    } else {
        "bash"
    };
    let command = format!("{} {}", translation_command, locales_path);
    println!("{}", command);
    let output = Command::new(program)
        .current_dir(dir_path)
        .args(&["-c", &command])
        .output()
        .expect("failed to execute mkdir");
    println!("output: {:?}", output);
}

fn add_key_to_messages_ts(
    path: String,
    ts_key: String,
    json_key: String,
) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new().read(true).open(path.clone())?;

    let mut lines: Vec<String> = Vec::new();
    let mut check_line = 0;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file");
    content.lines().enumerate().for_each(|(index, line)| {
        if line.contains("},") {
            check_line = index - 1;
            lines.push(format!("    {}: '{}'", ts_key, json_key));
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
        .open(path.clone())?;
    file.write_all(lines_content.as_bytes())?;

    Ok(())
}
fn add_translation_to_default_language(
    path: String,
    json_key: String,
    en_gb_value: String,
) -> Result<(), std::io::Error> {
    let en_gb_path = PathType::EnGbFile.create_path(path.clone());
    let content = read_to_string(en_gb_path.clone())?;
    let mut json_content: serde_json::Value = serde_json::from_str(&content)?;
    json_content
        .as_object_mut()
        .unwrap()
        .insert(json_key, serde_json::Value::String(en_gb_value));
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(en_gb_path)?;

    let formatter = PrettyFormatter::with_indent(b"\t");
    let mut buffer = Vec::new();
    let mut ser = serde_json::Serializer::with_formatter(&mut buffer, formatter);
    json_content.serialize(&mut ser).unwrap();

    let prettified = String::from_utf8(buffer).unwrap();
    file.write_all(prettified.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_new_key() {
        let path = String::from(r"C:\\Users\\ihm1we\\Development\\LeadManagement\\app.dtp-admin.frontend\\src");
        let ts_key = String::from("test");
        let json_key = String::from("test");
        let en_gb_value = String::from("    test");
        let result = TranslationHandler::add_new_key(path, ts_key, json_key, en_gb_value).unwrap();
    }
}
