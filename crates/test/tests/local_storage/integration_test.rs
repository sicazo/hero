use local_storage::StoreType;
use local_storage::handler::{get_store, update_store, remove_store};

#[test]
fn test_store_type_conversion() {
    // Test that we can convert a string to a StoreType
    let store_type = StoreType::from_string("settings".to_string());
    assert_eq!(store_type, StoreType::SettingsStoreType);
    
    let store_type = StoreType::from_string("translation".to_string());
    assert_eq!(store_type, StoreType::TranslationStoreType);
    
    let store_type = StoreType::from_string("location".to_string());
    assert_eq!(store_type, StoreType::LocationStoreType);
    
    // Test with an unknown store type (should default to SettingsStoreType)
    let store_type = StoreType::from_string("unknown".to_string());
    assert_eq!(store_type, StoreType::SettingsStoreType);
}

// Note: The following tests are commented out because they would modify actual storage files
// In a real scenario, you would use mocking or a test-specific storage location

/*
#[test]
fn test_get_store() {
    // Test that we can get a store
    let store_json = get_store("settings".to_string());
    assert!(!store_json.is_empty());
}

#[test]
fn test_update_and_remove_store() {
    // Test that we can update and remove a store
    let test_value = r#"{"version":1.0,"theme":"dark"}"#;
    
    // Update the store
    update_store("settings".to_string(), test_value.to_string());
    
    // Get the store and verify it was updated
    let store_json = get_store("settings".to_string());
    assert!(store_json.contains("dark"));
    
    // Remove the store
    remove_store("settings".to_string());
    
    // Get the store and verify it was reset
    let store_json = get_store("settings".to_string());
    assert!(!store_json.contains("dark"));
}
*/

// Add more integration tests that use only public APIs