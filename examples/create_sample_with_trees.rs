use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a sample database with multiple trees for testing
    let db_path = "sample_with_trees.db";

    // Remove existing database if it exists
    if fs::metadata(db_path).is_ok() {
        fs::remove_dir_all(db_path)?;
    }

    println!(
        "Creating sample database with multiple trees at: {}",
        db_path
    );
    let db = sled::open(db_path)?;

    // Default tree - basic user data
    println!("Populating default tree with user data...");
    db.insert(b"user:001", b"John Doe")?;
    db.insert(b"user:002", b"Jane Smith")?;
    db.insert(b"user:003", b"Bob Johnson")?;
    db.insert(b"user:004", b"Alice Brown")?;
    db.insert(b"user:005", b"Charlie Wilson")?;
    db.insert(b"admin:root", b"System Administrator")?;

    // Settings tree
    println!("Creating settings tree...");
    let settings_tree = db.open_tree(b"settings")?;
    settings_tree.insert(b"app.theme", b"dark")?;
    settings_tree.insert(b"app.language", b"en-US")?;
    settings_tree.insert(b"app.timeout", b"3600")?;
    settings_tree.insert(b"ui.sidebar_width", b"250")?;
    settings_tree.insert(b"ui.show_tooltips", b"true")?;
    settings_tree.insert(b"security.session_timeout", b"1800")?;
    settings_tree.insert(b"security.max_attempts", b"3")?;
    settings_tree.insert(b"db.backup_interval", b"86400")?;
    settings_tree.insert(b"logging.level", b"INFO")?;
    settings_tree.insert(b"logging.file_rotation", b"daily")?;

    // Sessions tree
    println!("Creating sessions tree...");
    let sessions_tree = db.open_tree(b"sessions")?;
    sessions_tree.insert(
        b"sess_abc123def456",
        &br#"{"user_id": "001", "created": "2024-01-01T10:00:00Z", "ip": "192.168.1.100"}"#[..],
    )?;
    sessions_tree.insert(
        b"sess_def456ghi789",
        &br#"{"user_id": "002", "created": "2024-01-01T11:30:00Z", "ip": "192.168.1.101"}"#[..],
    )?;
    sessions_tree.insert(
        b"sess_ghi789jkl012",
        &br#"{"user_id": "003", "created": "2024-01-01T14:15:00Z", "ip": "192.168.1.102"}"#[..],
    )?;
    sessions_tree.insert(
        b"sess_jkl012mno345",
        &br#"{"user_id": "001", "created": "2024-01-01T16:45:00Z", "ip": "10.0.0.50"}"#[..],
    )?;
    sessions_tree.insert(
        b"sess_mno345pqr678",
        &br#"{"user_id": "004", "created": "2024-01-01T18:20:00Z", "ip": "172.16.0.25"}"#[..],
    )?;

    // Cache tree - temporary data with various types
    println!("Creating cache tree...");
    let cache_tree = db.open_tree(b"cache")?;
    cache_tree.insert(
        b"weather:london",
        &br#"{"temp": 15, "humidity": 72, "condition": "cloudy", "timestamp": 1704110400}"#[..],
    )?;
    cache_tree.insert(
        b"weather:paris",
        &br#"{"temp": 18, "humidity": 65, "condition": "sunny", "timestamp": 1704110400}"#[..],
    )?;
    cache_tree.insert(
        b"weather:tokyo",
        &br#"{"temp": 22, "humidity": 80, "condition": "rainy", "timestamp": 1704110400}"#[..],
    )?;
    cache_tree.insert(
        b"api_response:github:user001",
        &br#"{"login": "johndoe", "id": 12345, "avatar_url": "https://github.com/avatars/u/12345"}"#[..],
    )?;
    cache_tree.insert(b"computed:factorial:10", b"3628800")?;
    cache_tree.insert(b"computed:fibonacci:20", b"6765")?;
    cache_tree.insert(
        b"temp:upload:file001",
        &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],
    )?; // PNG header
    cache_tree.insert(b"temp:processing:job123", b"in_progress")?;

    // Logs tree - application logs
    println!("Creating logs tree...");
    let logs_tree = db.open_tree(b"logs")?;
    logs_tree.insert(b"2024-01-01T10:00:00Z", b"[INFO] Application started")?;
    logs_tree.insert(
        b"2024-01-01T10:00:15Z",
        &b"[INFO] Database connection established"[..],
    )?;
    logs_tree.insert(
        b"2024-01-01T10:05:30Z",
        &b"[DEBUG] User 001 logged in from 192.168.1.100"[..],
    )?;
    logs_tree.insert(
        b"2024-01-01T10:15:45Z",
        &b"[WARN] High memory usage detected: 85%"[..],
    )?;
    logs_tree.insert(
        b"2024-01-01T10:30:20Z",
        &b"[ERROR] Failed to connect to external API: timeout"[..],
    )?;
    logs_tree.insert(
        b"2024-01-01T10:45:10Z",
        &b"[INFO] Backup completed successfully"[..],
    )?;
    logs_tree.insert(b"2024-01-01T11:00:00Z", b"[DEBUG] Cleanup task started")?;
    logs_tree.insert(
        b"2024-01-01T11:15:33Z",
        &b"[INFO] User 002 updated profile"[..],
    )?;
    logs_tree.insert(
        b"2024-01-01T11:30:45Z",
        &b"[WARN] Disk space low: 15% remaining"[..],
    )?;
    logs_tree.insert(
        b"2024-01-01T12:00:00Z",
        &b"[INFO] Daily statistics generated"[..],
    )?;

    // Metrics tree - performance and usage metrics
    println!("Creating metrics tree...");
    let metrics_tree = db.open_tree(b"metrics")?;
    metrics_tree.insert(b"cpu_usage:2024-01-01T10:00", b"45.2")?;
    metrics_tree.insert(b"cpu_usage:2024-01-01T10:01", b"52.1")?;
    metrics_tree.insert(b"cpu_usage:2024-01-01T10:02", b"38.7")?;
    metrics_tree.insert(b"memory_usage:2024-01-01T10:00", b"1024.5")?;
    metrics_tree.insert(b"memory_usage:2024-01-01T10:01", b"1156.8")?;
    metrics_tree.insert(b"memory_usage:2024-01-01T10:02", b"987.3")?;
    metrics_tree.insert(b"request_count:total", b"15847")?;
    metrics_tree.insert(b"request_count:today", b"234")?;
    metrics_tree.insert(b"error_count:total", b"45")?;
    metrics_tree.insert(b"error_count:today", b"2")?;
    metrics_tree.insert(b"response_time:avg", b"125.4")?;
    metrics_tree.insert(b"response_time:p95", b"450.2")?;

    // Configuration tree - structured configuration data
    println!("Creating configuration tree...");
    let config_tree = db.open_tree(b"configuration")?;
    config_tree.insert(b"database.host", b"localhost")?;
    config_tree.insert(b"database.port", b"5432")?;
    config_tree.insert(b"database.name", b"production")?;
    config_tree.insert(b"database.pool_size", b"10")?;
    config_tree.insert(b"server.host", b"0.0.0.0")?;
    config_tree.insert(b"server.port", b"8080")?;
    config_tree.insert(b"server.workers", b"4")?;
    config_tree.insert(b"redis.url", b"redis://localhost:6379")?;
    config_tree.insert(b"email.smtp_server", b"smtp.example.com")?;
    config_tree.insert(b"email.port", b"587")?;
    config_tree.insert(b"features.new_ui", b"enabled")?;
    config_tree.insert(b"features.beta_features", b"disabled")?;

    // Binary data tree - various binary formats
    println!("Creating binary data tree...");
    let binary_tree = db.open_tree(b"binary_data")?;

    // Small PNG header
    binary_tree.insert(
        b"image:png:header",
        &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],
    )?;

    // JPEG header
    binary_tree.insert(b"image:jpeg:header", &[0xFF, 0xD8, 0xFF, 0xE0])?;

    // PDF header
    binary_tree.insert(b"document:pdf:header", b"%PDF-1.4")?;

    // Some binary data
    let binary_data: Vec<u8> = (0..=255).collect();
    binary_tree.insert(b"data:bytes:0-255", binary_data.as_slice())?;

    // Encrypted-looking data (random bytes)
    let encrypted_data = vec![
        0x3F, 0xA2, 0x7B, 0x15, 0xC8, 0x91, 0x4E, 0x6D, 0x8A, 0x42, 0xF3, 0x7C, 0x19, 0xB5, 0x96,
        0x2E,
    ];
    binary_tree.insert(b"encrypted:sample", encrypted_data.as_slice())?;

    // UUID-like data
    binary_tree.insert(
        b"uuid:sample1",
        &b"550e8400-e29b-41d4-a716-446655440000"[..],
    )?;
    binary_tree.insert(
        b"uuid:sample2",
        &b"6ba7b810-9dad-11d1-80b4-00c04fd430c8"[..],
    )?;

    // Hash-like data
    binary_tree.insert(b"hash:md5:sample", &b"5d41402abc4b2a76b9719d911017c592"[..])?;
    binary_tree.insert(
        b"hash:sha256:sample",
        &b"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"[..],
    )?;

    // Test data tree - for development and testing
    println!("Creating test data tree...");
    let test_tree = db.open_tree(b"test_data")?;

    // Edge cases
    test_tree.insert(b"empty_value", b"")?;
    test_tree.insert(b"", b"empty_key")?;
    test_tree.insert(b"null_byte", &[0x00])?;
    test_tree.insert(b"unicode", "Hello, 世界! 🌍".as_bytes())?;
    test_tree.insert(
        b"very_long_key_that_exceeds_normal_length_expectations_and_tests_handling_of_long_keys",
        b"short_value",
    )?;
    test_tree.insert(b"short", &b"This is a very long value that exceeds normal expectations and tests the handling of large values in the database system. It contains multiple sentences and should help test display and formatting capabilities."[..])?;

    // JSON-like structures
    test_tree.insert(
        b"json:valid",
        &br#"{"name": "Test", "value": 42, "active": true, "nested": {"key": "value"}}"#[..],
    )?;
    test_tree.insert(
        b"json:array",
        &br#"[1, 2, 3, "test", {"nested": true}]"#[..],
    )?;
    test_tree.insert(
        b"json:malformed",
        &br#"{"name": "Test", "value": 42, "missing_close""#[..],
    )?;

    // XML-like data
    test_tree.insert(
        b"xml:sample",
        &br#"<root><item id="1">Value 1</item><item id="2">Value 2</item></root>"#[..],
    )?;

    // Numbers in various formats
    test_tree.insert(b"number:int", b"42")?;
    test_tree.insert(b"number:float", b"3.14159")?;
    test_tree.insert(b"number:scientific", b"1.23e-4")?;
    test_tree.insert(b"number:hex", b"0xDEADBEEF")?;
    test_tree.insert(b"number:binary", b"0b10101010")?;

    // Flush all trees to ensure data is persisted
    println!("Flushing all data to disk...");
    db.flush()?;
    settings_tree.flush()?;
    sessions_tree.flush()?;
    cache_tree.flush()?;
    logs_tree.flush()?;
    metrics_tree.flush()?;
    config_tree.flush()?;
    binary_tree.flush()?;
    test_tree.flush()?;

    // Display summary
    println!("\n=== Database Creation Summary ===");
    println!("Database path: {}", db_path);
    println!("Default tree entries: {}", db.len());
    println!("Settings tree entries: {}", settings_tree.len());
    println!("Sessions tree entries: {}", sessions_tree.len());
    println!("Cache tree entries: {}", cache_tree.len());
    println!("Logs tree entries: {}", logs_tree.len());
    println!("Metrics tree entries: {}", metrics_tree.len());
    println!("Configuration tree entries: {}", config_tree.len());
    println!("Binary data tree entries: {}", binary_tree.len());
    println!("Test data tree entries: {}", test_tree.len());

    let total_entries = db.len()
        + settings_tree.len()
        + sessions_tree.len()
        + cache_tree.len()
        + logs_tree.len()
        + metrics_tree.len()
        + config_tree.len()
        + binary_tree.len()
        + test_tree.len();
    println!("Total entries across all trees: {}", total_entries);

    println!("\n=== Trees Created ===");
    println!("• default       - User data and basic entries");
    println!("• settings      - Application settings and preferences");
    println!("• sessions      - User session data with JSON structures");
    println!("• cache         - Temporary cached data and API responses");
    println!("• logs          - Application logs with timestamps");
    println!("• metrics       - Performance metrics and statistics");
    println!("• configuration - Structured configuration parameters");
    println!("• binary_data   - Various binary formats and data types");
    println!("• test_data     - Edge cases and test scenarios");

    println!("\nDatabase created successfully! You can now test sledoview with:");
    println!("cargo run -- {}", db_path);

    Ok(())
}
