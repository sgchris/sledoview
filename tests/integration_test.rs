mod common;

use assert_cmd::Command;
use predicates::prelude::*;
use sledoview::db::SledViewer;
use sledoview::validator::DatabaseValidator;

#[test]
fn test_validator_valid_database() {
    let temp_dir = common::create_test_db();
    let validator = DatabaseValidator::new(temp_dir.path());
    assert!(validator.validate().is_ok());
}

#[test]
fn test_validator_nonexistent_database() {
    let validator = DatabaseValidator::new(std::path::Path::new("/nonexistent/path"));
    assert!(validator.validate().is_err());
}

#[test]
fn test_validator_file_instead_of_directory() {
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let validator = DatabaseValidator::new(temp_file.path());
    assert!(validator.validate().is_err());
}

#[test]
fn test_sled_viewer_count() {
    let temp_dir = common::create_test_db();
    let viewer = SledViewer::new(temp_dir.path()).unwrap();
    let count = viewer.count().unwrap();
    assert_eq!(count, 10); // We inserted 10 items in create_test_db
}

#[test]
fn test_sled_viewer_list_keys_all() {
    let temp_dir = common::create_test_db();
    let viewer = SledViewer::new(temp_dir.path()).unwrap();
    let keys = viewer.list_keys("*", false).unwrap();
    assert_eq!(keys.len(), 10);
    assert!(keys.contains(&"user_001".to_string()));
    assert!(keys.contains(&"config_theme".to_string()));
}

#[test]
fn test_sled_viewer_list_keys_pattern() {
    let temp_dir = common::create_test_db();
    let viewer = SledViewer::new(temp_dir.path()).unwrap();
    let keys = viewer.list_keys("user_*", false).unwrap();
    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&"user_001".to_string()));
    assert!(keys.contains(&"user_002".to_string()));
    assert!(keys.contains(&"user_003".to_string()));
}

#[test]
fn test_sled_viewer_list_keys_regex() {
    let temp_dir = common::create_test_db();
    let viewer = SledViewer::new(temp_dir.path()).unwrap();
    let keys = viewer.list_keys(r"user_\d+", true).unwrap();
    assert_eq!(keys.len(), 3);
}

#[test]
fn test_sled_viewer_get_key() {
    let temp_dir = common::create_test_db();
    let viewer = SledViewer::new(temp_dir.path()).unwrap();
    let key_info = viewer.get_key("user_001").unwrap();
    assert_eq!(key_info.key, "user_001");
    assert_eq!(key_info.value, "John Doe");
    assert_eq!(key_info.size, 8);
    assert!(key_info.is_utf8);
}

#[test]
fn test_sled_viewer_get_nonexistent_key() {
    let temp_dir = common::create_test_db();
    let viewer = SledViewer::new(temp_dir.path()).unwrap();
    let result = viewer.get_key("nonexistent_key");
    assert!(result.is_err());
}

#[test]
fn test_sled_viewer_search_values_pattern() {
    let temp_dir = common::create_test_db();
    let viewer = SledViewer::new(temp_dir.path()).unwrap();
    let results = viewer.search_values("*@example.com", false).unwrap();
    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|r| r.key == "email_john"));
    assert!(results.iter().any(|r| r.key == "email_jane"));
}

#[test]
fn test_sled_viewer_search_values_regex() {
    let temp_dir = common::create_test_db();
    let viewer = SledViewer::new(temp_dir.path()).unwrap();
    let results = viewer.search_values(r"\w+@example\.com", true).unwrap();
    assert_eq!(results.len(), 2);
}

#[test]
fn test_empty_database() {
    let temp_dir = common::create_empty_test_db();
    let viewer = SledViewer::new(temp_dir.path()).unwrap();
    assert_eq!(viewer.count().unwrap(), 0);
    assert_eq!(viewer.list_keys("*", false).unwrap().len(), 0);
}

#[test]
fn test_binary_data() {
    let temp_dir = common::create_test_db();
    let viewer = SledViewer::new(temp_dir.path()).unwrap();
    let key_info = viewer.get_key("data_binary").unwrap();
    assert_eq!(key_info.key, "data_binary");
    assert_eq!(key_info.size, 5);
    assert!(!key_info.is_utf8);
}

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("sledoview").unwrap();
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "CLI tool for viewing and managing SLED databases",
    ));
}

#[test]
fn test_cli_nonexistent_database() {
    let mut cmd = Command::cargo_bin("sledoview").unwrap();
    cmd.arg("/nonexistent/database");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Database file not found"));
}
