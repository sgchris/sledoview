use tempfile::TempDir;

/// Creates a temporary SLED database for testing
pub fn create_test_db() -> TempDir {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    {
        let db = sled::open(temp_dir.path()).expect("Failed to create test database");

        // Populate with test data
        db.insert(b"user_001", b"John Doe").unwrap();
        db.insert(b"user_002", b"Jane Smith").unwrap();
        db.insert(b"user_003", b"Bob Johnson").unwrap();
        db.insert(b"config_theme", b"dark").unwrap();
        db.insert(b"config_language", b"en-US").unwrap();
        db.insert(b"session_abc123", b"2024-01-01T10:00:00Z")
            .unwrap();
        db.insert(b"session_def456", b"2024-01-01T11:00:00Z")
            .unwrap();
        db.insert(b"email_john", b"john@example.com").unwrap();
        db.insert(b"email_jane", b"jane@example.com").unwrap();
        db.insert(b"data_binary", &[0u8, 1, 2, 3, 255]).unwrap();

        // Flush to ensure data is written
        db.flush().unwrap();
    } // Database is dropped here, releasing the lock

    temp_dir
}

/// Creates an empty temporary SLED database
pub fn create_empty_test_db() -> TempDir {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    {
        let db = sled::open(temp_dir.path()).expect("Failed to create test database");
        db.flush().unwrap();
    } // Database is dropped here, releasing the lock
    temp_dir
}

/// Creates a test database with specific data
pub fn create_custom_test_db(data: &[(&str, &str)]) -> TempDir {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    {
        let db = sled::open(temp_dir.path()).expect("Failed to create test database");

        for (key, value) in data {
            db.insert(key.as_bytes(), value.as_bytes()).unwrap();
        }

        db.flush().unwrap();
    } // Database is dropped here, releasing the lock
    temp_dir
}
