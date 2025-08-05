use local_storage::handler::create_storage;

#[test]
fn test_path() {
    create_storage().expect("Failed to create storage");
}