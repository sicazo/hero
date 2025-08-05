use translation_handler::TranslationHandler;
use std::path::Path;

#[test]
fn test_translation_handler_creation() {
    // Test that we can create a TranslationHandler instance
    let handler = TranslationHandler::new();
    assert!(handler.is_ok());
}

#[test]
fn test_path_type_conversion() {
    // Test that we can use the PathType enum
    use translation_handler::frontend::PathType;
    
    let path = "test/path";
    let path_type = PathType::MessagesTs(path.to_string());
    
    match path_type {
        PathType::MessagesTs(p) => assert_eq!(p, path),
        _ => panic!("Wrong path type"),
    }
}

// Add more integration tests that use only public APIs