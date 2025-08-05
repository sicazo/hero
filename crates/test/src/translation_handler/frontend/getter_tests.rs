use translation_handler::frontend::getter::{remove_key_from_language_jsons, remove_key_from_messages_ts};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tempfile::TempDir;

fn setup_test_dir() -> TempDir {
    tempfile::tempdir().expect("failed to create temporary directory")
}

fn create_json_file(dir: &Path, filename: &str, content: &str) -> std::io::Result<()> {
    let file_path = dir.join(filename);
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn read_file_content(path: &Path) -> String {
    fs::read_to_string(path).expect("failed to read file")
}

#[test]
fn test_remove_key_from_language_jsons_basic() {
    let temp_dir = setup_test_dir();
    let locales_path = temp_dir.path().to_string_lossy().to_string();

    // Create test JSON files
    let json_content = r#"{
  "hello": "Hello",
  "goodbye": "Goodbye",
  "welcome": "Welcome"
}"#;
    create_json_file(temp_dir.path(), "en.json", json_content).unwrap();
    create_json_file(temp_dir.path(), "es.json", json_content).unwrap();

    // Remove keys
    let keys_to_remove = vec!["goodbye".to_string()];
    let result = remove_key_from_language_jsons(locales_path, keys_to_remove);

    assert!(result.is_ok());

    // Verify content
    let en_content = read_file_content(&temp_dir.path().join("en.json"));
    assert!(!en_content.contains("goodbye"));
    assert!(en_content.contains("hello"));
    assert!(en_content.contains("welcome"));

    let es_content = read_file_content(&temp_dir.path().join("es.json"));
    assert!(!es_content.contains("goodbye"));
    assert!(es_content.contains("hello"));
    assert!(es_content.contains("welcome"));
}

#[test]
fn test_remove_key_from_language_jsons_multiple_keys() {
    let temp_dir = setup_test_dir();
    let locales_path = temp_dir.path().to_string_lossy().to_string();

    let json_content = r#"{
  "key1": "Value 1",
  "key2": "Value 2",
  "key3": "Value 3",
  "key4": "Value 4"
}"#;
    create_json_file(temp_dir.path(), "test.json", json_content).unwrap();

    let keys_to_remove = vec!["key1".to_string(), "key3".to_string()];
    let result = remove_key_from_language_jsons(locales_path, keys_to_remove);

    assert!(result.is_ok());

    let content = read_file_content(&temp_dir.path().join("test.json"));
    assert!(!content.contains("key1"));
    assert!(!content.contains("key3"));
    assert!(content.contains("key2"));
    assert!(content.contains("key4"));
}

#[test]
fn test_remove_key_from_language_jsons_nonexistent_keys() {
    let temp_dir = setup_test_dir();
    let locales_path = temp_dir.path().to_string_lossy().to_string();

    let json_content = r#"{
  "existing_key": "Value"
}"#;
    create_json_file(temp_dir.path(), "test.json", json_content).unwrap();

    let keys_to_remove = vec!["nonexistent_key".to_string()];
    let result = remove_key_from_language_jsons(locales_path, keys_to_remove);

    assert!(result.is_ok());

    let content = read_file_content(&temp_dir.path().join("test.json"));
    assert!(content.contains("existing_key"));
}

#[test]
fn test_remove_key_from_language_jsons_tab_indentation() {
    let temp_dir = setup_test_dir();
    let locales_path = temp_dir.path().to_string_lossy().to_string();

    let json_content = "{\n\t\"hello\": \"Hello\",\n\t\"goodbye\": \"Goodbye\"\n}";
    create_json_file(temp_dir.path(), "test.json", json_content).unwrap();

    let keys_to_remove = vec!["goodbye".to_string()];
    let result = remove_key_from_language_jsons(locales_path, keys_to_remove);

    assert!(result.is_ok());

    let content = read_file_content(&temp_dir.path().join("test.json"));
    assert!(!content.contains("goodbye"));
    assert!(content.contains("hello"));
    // Should preserve tab indentation
    assert!(content.contains("\t"));
}

#[test]
fn test_remove_key_from_language_jsons_empty_json() {
    let temp_dir = setup_test_dir();
    let locales_path = temp_dir.path().to_string_lossy().to_string();

    let json_content = "{}";
    create_json_file(temp_dir.path(), "empty.json", json_content).unwrap();

    let keys_to_remove = vec!["any_key".to_string()];
    let result = remove_key_from_language_jsons(locales_path, keys_to_remove);

    assert!(result.is_ok());

    let content = read_file_content(&temp_dir.path().join("empty.json"));
    assert_eq!(content.trim(), "{\n}");
}

#[test]
fn test_remove_key_from_language_jsons_invalid_json() {
    let temp_dir = setup_test_dir();
    let locales_path = temp_dir.path().to_string_lossy().to_string();

    let invalid_json = "{ invalid json content";
    create_json_file(temp_dir.path(), "invalid.json", invalid_json).unwrap();

    let keys_to_remove = vec!["any_key".to_string()];
    let result = remove_key_from_language_jsons(locales_path, keys_to_remove);

    assert!(result.is_err());
}

#[test]
fn test_remove_key_from_language_jsons_no_files() {
    let temp_dir = setup_test_dir();
    let locales_path = temp_dir.path().to_string_lossy().to_string();

    let keys_to_remove = vec!["any_key".to_string()];
    let result = remove_key_from_language_jsons(locales_path, keys_to_remove);

    // Should succeed even with no files
    assert!(result.is_ok());
}

#[test]
fn test_remove_key_from_messages_ts_basic() {
    let temp_dir = setup_test_dir();
    let file_path = temp_dir.path().join("messages.ts");

    let ts_content = r#"import { defineLocales } from 'some-lib';

export default defineLocales({
  hello: "Hello",
  goodbye: "Goodbye",
  welcome: "Welcome",
  locales: {
    en: "English"
  }
});"#;

    let mut file = File::create(&file_path).unwrap();
    file.write_all(ts_content.as_bytes()).unwrap();

    let keys_to_remove = vec!["goodbye".to_string()];
    let result =
        remove_key_from_messages_ts(file_path.to_string_lossy().to_string(), keys_to_remove);

    assert!(result.is_ok());

    let content = read_file_content(&file_path);
    assert!(content.contains("hello"));
    assert!(!content.contains("goodbye"));
    assert!(content.contains("welcome"));
    assert!(content.contains("locales"));
}

#[test]
fn test_remove_key_from_messages_ts_multiple_keys() {
    let temp_dir = setup_test_dir();
    let file_path = temp_dir.path().join("messages.ts");

    let ts_content = r#"export default defineLocales({
  key1: "Value 1",
  key2: "Value 2",
  key3: "Value 3", 
  key4: "Value 4",
  locales: {
    en: "English"
  }
});"#;

    let mut file = File::create(&file_path).unwrap();
    file.write_all(ts_content.as_bytes()).unwrap();

    let keys_to_remove = vec!["key1".to_string(), "key3".to_string()];
    let result =
        remove_key_from_messages_ts(file_path.to_string_lossy().to_string(), keys_to_remove);

    assert!(result.is_ok());

    let content = read_file_content(&file_path);
    assert!(!content.contains("key1"));
    assert!(content.contains("key2"));
    assert!(!content.contains("key3"));
    assert!(content.contains("key4"));
}

#[test]
fn test_remove_key_from_messages_ts_single_quotes() {
    let temp_dir = setup_test_dir();
    let file_path = temp_dir.path().join("messages.ts");

    let ts_content = r#"export default defineLocales({
  hello: 'Hello World',
  goodbye: 'Goodbye World',
  locales: {
    en: 'English'
  }
});"#;

    let mut file = File::create(&file_path).unwrap();
    file.write_all(ts_content.as_bytes()).unwrap();

    let keys_to_remove = vec!["goodbye".to_string()];
    let result =
        remove_key_from_messages_ts(file_path.to_string_lossy().to_string(), keys_to_remove);

    assert!(result.is_ok());

    let content = read_file_content(&file_path);
    assert!(content.contains("hello"));
    assert!(!content.contains("goodbye"));
}

#[test]
fn test_remove_key_from_messages_ts_nonexistent_keys() {
    let temp_dir = setup_test_dir();
    let file_path = temp_dir.path().join("messages.ts");

    let ts_content = r#"export default defineLocales({
  existing_key: "Value",
  locales: {
    en: "English"
  }
});"#;

    let mut file = File::create(&file_path).unwrap();
    file.write_all(ts_content.as_bytes()).unwrap();

    let keys_to_remove = vec!["nonexistent_key".to_string()];
    let result =
        remove_key_from_messages_ts(file_path.to_string_lossy().to_string(), keys_to_remove);

    assert!(result.is_ok());

    let content = read_file_content(&file_path);
    assert!(content.contains("existing_key"));
}

#[test]
fn test_remove_key_from_messages_ts_no_define_locales() {
    let temp_dir = setup_test_dir();
    let file_path = temp_dir.path().join("messages.ts");

    let ts_content = r#"const messages = {
  hello: "Hello"
};"#;

    let mut file = File::create(&file_path).unwrap();
    file.write_all(ts_content.as_bytes()).unwrap();

    let keys_to_remove = vec!["hello".to_string()];
    let result =
        remove_key_from_messages_ts(file_path.to_string_lossy().to_string(), keys_to_remove);

    assert!(result.is_err());
}

#[test]
fn test_remove_key_from_messages_ts_no_locales() {
    let temp_dir = setup_test_dir();
    let file_path = temp_dir.path().join("messages.ts");

    let ts_content = r#"export default defineLocales({
  hello: "Hello"
});"#;

    let mut file = File::create(&file_path).unwrap();
    file.write_all(ts_content.as_bytes()).unwrap();

    let keys_to_remove = vec!["hello".to_string()];
    let result =
        remove_key_from_messages_ts(file_path.to_string_lossy().to_string(), keys_to_remove);

    assert!(result.is_err());
}

#[test]
fn test_remove_key_from_messages_ts_preserves_formatting() {
    let temp_dir = setup_test_dir();
    let file_path = temp_dir.path().join("messages.ts");

    let ts_content = r#"export default defineLocales({
  hello: "Hello",
  
  // Comment here
  goodbye: "Goodbye",
  welcome: "Welcome",
  locales: {
    en: "English"
  }
});"#;

    let mut file = File::create(&file_path).unwrap();
    file.write_all(ts_content.as_bytes()).unwrap();

    let keys_to_remove = vec!["goodbye".to_string()];
    let result =
        remove_key_from_messages_ts(file_path.to_string_lossy().to_string(), keys_to_remove);

    assert!(result.is_ok());

    let content = read_file_content(&file_path);
    assert!(content.contains("hello"));
    assert!(!content.contains("goodbye"));
    assert!(content.contains("welcome"));
    assert!(content.contains("// Comment here"));
}

#[test]
fn test_remove_key_from_messages_ts_file_not_found() {
    let keys_to_remove = vec!["any_key".to_string()];
    let result = remove_key_from_messages_ts(
        "/nonexistent/path/messages.ts".to_string(),
        keys_to_remove,
    );

    assert!(result.is_err());
}

#[test]
fn test_remove_key_from_language_jsons_complex_values() {
    let temp_dir = setup_test_dir();
    let locales_path = temp_dir.path().to_string_lossy().to_string();

    let json_content = r#"{
  "simple": "Simple value",
  "nested": {
    "key": "Nested value"
  },
  "array": ["item1", "item2"],
  "number": 42,
  "boolean": true,
  "to_remove": "This should be removed"
}"#;
    create_json_file(temp_dir.path(), "complex.json", json_content).unwrap();

    let keys_to_remove = vec!["to_remove".to_string()];
    let result = remove_key_from_language_jsons(locales_path, keys_to_remove);

    assert!(result.is_ok());

    let content = read_file_content(&temp_dir.path().join("complex.json"));
    assert!(!content.contains("to_remove"));
    assert!(content.contains("simple"));
    assert!(content.contains("nested"));
    assert!(content.contains("array"));
    assert!(content.contains("number"));
    assert!(content.contains("boolean"));
}