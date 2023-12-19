use std::collections::HashMap;
use std::fs::read_to_string;
use tracing::info;

pub struct TranslationHandler;

enum PathType {
    MessageTsFile,
    TranslationDirectory,
    TranslationExportFile,
}

impl TranslationHandler {
    fn create_sub_path(path: String, path_type: PathType) -> String {
        match path_type {
            PathType::MessageTsFile => {
                format!("{}/messages.ts", path)
            }
            PathType::TranslationDirectory => {
                format!("{}/locales", path)
            }
            PathType::TranslationExportFile => {
                format!("{}/locales/locales.ts", path)
            }
        }
    }

    fn extract_language_code(line: &str, language_code_regex: &regex::Regex) -> Option<String> {
        let captures = language_code_regex.captures(line)?;
        let language_code = captures.get(1)?;
        Some(language_code.as_str().to_string())
    }

    pub fn extract_language_codes_from_locales(path: String) -> Vec<String> {
        let sub_path = Self::create_sub_path(path, PathType::TranslationExportFile);
        info!("Reading locales.ts file in {}", sub_path);
        let language_code_regex = regex::Regex::new(r"'(\w{2}-\w{2})").unwrap();
        let file_content = read_to_string(&sub_path)
            .expect(format!("Failed to read locales.ts file in {}", sub_path).as_str());
        let file_content = file_content.lines().skip(1);

        let language_codes: Vec<String> = file_content
            .filter_map(|line| Self::extract_language_code(line, &language_code_regex))
            .collect();

        info!("Finished reading locales.ts file in {}", sub_path);
        language_codes
    }

    pub async fn get_key_values_from_messages_ts(path: &str) -> HashMap<String, String> {
        let sub_path = Self::create_sub_path(path.to_string(), PathType::MessageTsFile);
        let file_content = read_to_string(&sub_path)
            .expect(format!("Failed to read messages.ts file in {}", sub_path).as_str());
        let mut mappings = HashMap::new();
        info!("Reading messages.ts file in {}", sub_path);
        let key_value_regex = regex::Regex::new(r#"(\w+): '(.*)'"#).unwrap();
        for capture in key_value_regex.captures_iter(&file_content) {
            let key = capture.get(1).unwrap().as_str().to_string();
            let value = capture.get(2).unwrap().as_str().to_string();
            mappings.insert(key, value);
        }
        mappings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_languages_from_locales() {
        let path = String::from("/Users/marius/Developer/Github/translation_hero/testfiles");
        let languages = TranslationHandler::extract_language_codes_from_locales(path);
        println!("{:?}", languages)
    }
}
