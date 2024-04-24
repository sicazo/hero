#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::{frontend::PathType, TranslationHandler};
use db::prisma::settings::Data;
use local_storage::stores::translation_store::TranslationEntry;
use std::fs::OpenOptions;
use std::io::{Read, Write};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::process::Command;

impl TranslationHandler {
    pub async fn add_new_frontend_key(
        path: String,
        ts_key: String,
        json_key: String,
        en_gb_value: String,
        settings: Data,
    ) -> Result<Vec<TranslationEntry>, std::io::Error> {
        let messages_ts_path = PathType::MessageTsFile.create_path(path.clone());

        add_key_to_messages_ts(messages_ts_path, ts_key.clone(), json_key.clone())?;

        add_translation_to_default_language(path.clone(), json_key, en_gb_value)?;

        if settings.translate_new_strings {
            run_translation_command(&path.clone(), settings.translation_command);
        }

        let translations =
            TranslationHandler::get_frontend_translations(path.clone().as_str()).await;

        Ok(translations)
    }
}
pub fn run_translation_command(dir_path: &str, translation_command: String) {
    let locales_path = PathType::TranslationDirectory.create_path(dir_path.to_string());
    let program = if cfg!(target_os = "windows") {
        "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe"
    } else {
        "bash"
    };
    let command = format!("{} {}", translation_command, locales_path);
    println!("{}", command);

    #[cfg(target_os = "windows")]
    let output = Command::new(program)
        .current_dir(dir_path)
        .args(&["-c", &command])
        .creation_flags(0x08000000)
        .output()
        .expect("failed to execute mkdir");
    #[cfg(not(target_os = "windows"))]
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
pub fn add_translation_to_default_language(
    path: String,
    json_key: String,
    en_gb_value: String,
) -> Result<(), std::io::Error> {
    let en_gb_path = PathType::EnGbFile.create_path(path.clone());

    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(en_gb_path.clone())?;

    let mut lines: Vec<String> = Vec::new();
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file");

    content.lines().enumerate().for_each(|(_index, line)| {
        let trimmed_line = line.to_owned();

        if trimmed_line != "{" && trimmed_line != "}" {
            let mut updated_line = line.to_owned();
            if !updated_line.ends_with(",") {
                updated_line.push(',');
            }
            lines.push(updated_line);
        } else if trimmed_line == "}" {
            lines.push(format!(r#"    "{}": "{}""#, json_key, en_gb_value));
            lines.push(String::from(line))
        } else if trimmed_line == "{" {
            lines.push(trimmed_line);
        }
    });
    let lines_content: String = lines.join("\n");
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(en_gb_path.clone())?;
    file.write_all(lines_content.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_new_key() {}
}
