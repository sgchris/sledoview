use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an example database for testing
    let db_path = "example_db";

    // Remove existing database if it exists
    if fs::metadata(db_path).is_ok() {
        fs::remove_dir_all(db_path)?;
    }

    let db = sled::open(db_path)?;

    // Add some sample data
    db.insert(b"user_001", b"John Doe")?;
    db.insert(b"user_002", b"Jane Smith")?;
    db.insert(b"user_003", b"Bob Johnson")?;
    db.insert(b"user_admin", b"Administrator")?;

    db.insert(b"config_theme", b"dark")?;
    db.insert(b"config_language", b"en-US")?;
    db.insert(b"config_timeout", b"3600")?;

    db.insert(b"session_abc123", b"2024-01-01T10:00:00Z")?;
    db.insert(b"session_def456", b"2024-01-01T11:00:00Z")?;
    db.insert(b"session_ghi789", b"2024-01-01T12:00:00Z")?;

    db.insert(b"email_john", b"john.doe@example.com")?;
    db.insert(b"email_jane", b"jane.smith@gmail.com")?;
    db.insert(b"email_bob", b"bob@company.org")?;

    db.insert(
        b"data_json",
        br#"{"name": "Test User", "age": 30, "active": true}"#.as_slice(),
    )?;
    db.insert(b"data_binary", &[0u8, 1, 2, 3, 255])?;

    db.flush()?;

    println!("Example database created at: {}", db_path);
    println!("Total records: {}", db.len());

    Ok(())
}
