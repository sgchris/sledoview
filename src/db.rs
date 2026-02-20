use crate::error::SledoViewError;
use anyhow::Result;
use regex::Regex;
use sled::{Db, Tree};
use std::path::Path;

pub struct SledViewer {
    db: Db,
    selected_tree: Option<String>,
}

impl SledViewer {
    pub fn new(path: &Path) -> Result<Self> {
        let db = sled::open(path).map_err(|e| {
            // Sled lock errors are easy to miss; surface a clear message.
            if is_sled_lock_error(&e) {
                SledoViewError::DatabaseLocked {
                    path: path.display().to_string(),
                }
                .into()
            } else {
                anyhow::Error::from(e)
            }
        })?;
        Ok(Self {
            db,
            selected_tree: None,
        })
    }

    pub fn count(&self) -> Result<usize> {
        if let Some(tree_name) = &self.selected_tree {
            let tree = self.get_tree(tree_name)?;
            Ok(tree.len())
        } else {
            Ok(self.db.len())
        }
    }

    pub fn list_keys(&self, pattern: &str, is_regex: bool) -> Result<Vec<String>> {
        let mut keys = Vec::new();
        let regex = create_regex(pattern, is_regex)?;

        if let Some(tree_name) = &self.selected_tree {
            let tree = self.get_tree(tree_name)?;
            for result in &tree {
                let (key, _) = result?;
                let key_str = String::from_utf8_lossy(&key);
                if regex.is_match(&key_str) {
                    keys.push(key_str.to_string());
                }
            }
        } else {
            for result in self.db.iter() {
                let (key, _) = result?;
                let key_str = String::from_utf8_lossy(&key);
                if regex.is_match(&key_str) {
                    keys.push(key_str.to_string());
                }
            }
        }

        keys.sort();
        Ok(keys)
    }

    /// List raw key bytes, optionally filtered by pattern (glob or regex).
    ///
    /// Returns each key as its original `Vec<u8>`, which is required for keys
    /// that are not valid UTF-8 (e.g. big-endian encoded `u64` timestamps).
    pub fn list_keys_raw(&self, pattern: &str, is_regex: bool) -> Result<Vec<Vec<u8>>> {
        let mut keys: Vec<Vec<u8>> = Vec::new();
        let regex = create_regex(pattern, is_regex)?;

        if let Some(tree_name) = &self.selected_tree {
            let tree = self.get_tree(tree_name)?;
            for result in tree.iter() {
                let (key, _) = result?;
                let key_str = String::from_utf8_lossy(&key);
                if regex.is_match(&key_str) {
                    keys.push(key.to_vec());
                }
            }
        } else {
            for result in self.db.iter() {
                let (key, _) = result?;
                let key_str = String::from_utf8_lossy(&key);
                if regex.is_match(&key_str) {
                    keys.push(key.to_vec());
                }
            }
        }

        keys.sort();
        Ok(keys)
    }

    pub fn get_key(&self, key: &str) -> Result<KeyInfo> {
        let key_bytes = key.as_bytes();

        let value_opt = if let Some(tree_name) = &self.selected_tree {
            let tree = self.get_tree(tree_name)?;
            tree.get(key_bytes)?
        } else {
            self.db.get(key_bytes)?
        };

        if let Some(value) = value_opt {
            let value_str = String::from_utf8_lossy(&value);
            let size = value.len();

            Ok(KeyInfo {
                key: key.to_string(),
                value: value_str.to_string(),
                size,
                is_utf8: String::from_utf8(value.to_vec()).is_ok(),
            })
        } else {
            Err(SledoViewError::KeyNotFound {
                key: key.to_string(),
            }
            .into())
        }
    }

    /// Retrieve key info by raw bytes instead of a UTF-8 string.
    ///
    /// Required for keys that are stored as binary data (e.g. big-endian `u64`
    /// timestamps).  The `KeyInfo.key` field is populated via [`format_key_bytes`].
    pub fn get_key_bytes(&self, key_bytes: &[u8]) -> Result<KeyInfo> {
        let value_opt = match &self.selected_tree {
            Some(tree_name) => {
                let tree = self.get_tree(tree_name)?;
                tree.get(key_bytes)?
            }
            None => self.db.get(key_bytes)?,
        };

        match value_opt {
            Some(value) => {
                let is_utf8 = std::str::from_utf8(&value).is_ok();
                let value_str = String::from_utf8_lossy(&value).to_string();
                let size = value.len();
                Ok(KeyInfo {
                    key: format_key_bytes_full(key_bytes),
                    value: value_str,
                    size,
                    is_utf8,
                })
            }
            None => Err(SledoViewError::KeyNotFound {
                key: format_key_bytes(key_bytes),
            }
            .into()),
        }
    }

    /// Search all binary (non-UTF-8) keys for one whose full uppercase hex
    /// representation ends with `suffix` (case-insensitive).
    ///
    /// This lets users locate a key from the truncated display like
    /// `0000....61F8` by typing `get 61F8`.
    pub fn find_key_by_hex_suffix(&self, suffix: &str) -> Result<Option<KeyInfo>> {
        let suffix_upper = suffix.to_uppercase();

        // Helper closure: returns Some(KeyInfo) if this key matches.
        let try_match = |key: &[u8], value: &sled::IVec| -> Option<KeyInfo> {
            // Only consider binary (non-UTF-8) keys.
            if std::str::from_utf8(key).is_ok() {
                return None;
            }
            let hex: String = key.iter().map(|b| format!("{:02X}", b)).collect();
            if hex.ends_with(&suffix_upper) {
                let is_utf8 = std::str::from_utf8(value).is_ok();
                let value_str = String::from_utf8_lossy(value).to_string();
                Some(KeyInfo {
                    key: format_key_bytes_full(key),
                    value: value_str,
                    size: value.len(),
                    is_utf8,
                })
            } else {
                None
            }
        };

        match &self.selected_tree {
            Some(tree_name) => {
                let tree = self.get_tree(tree_name)?;
                for result in tree.iter() {
                    let (key, value) = result?;
                    if let Some(info) = try_match(&key, &value) {
                        return Ok(Some(info));
                    }
                }
            }
            None => {
                for result in self.db.iter() {
                    let (key, value) = result?;
                    if let Some(info) = try_match(&key, &value) {
                        return Ok(Some(info));
                    }
                }
            }
        }

        Ok(None)
    }

    pub fn search_values(&self, pattern: &str, is_regex: bool) -> Result<Vec<KeyValuePair>> {
        let mut results = Vec::new();

        let regex = create_regex(pattern, is_regex)?;

        if let Some(tree_name) = &self.selected_tree {
            let tree = self.get_tree(tree_name)?;
            for result in tree.iter() {
                let (key, value) = result?;
                let value_str = String::from_utf8_lossy(&value);
                if regex.is_match(&value_str) {
                    results.push(KeyValuePair {
                        key: format_key_bytes(&key),
                        value: value_str.to_string(),
                    });
                }
            }
        } else {
            for result in self.db.iter() {
                let (key, value) = result?;
                let value_str = String::from_utf8_lossy(&value);
                if regex.is_match(&value_str) {
                    results.push(KeyValuePair {
                        key: format_key_bytes(&key),
                        value: value_str.to_string(),
                    });
                }
            }
        }

        results.sort_by(|a, b| a.key.cmp(&b.key));
        Ok(results)
    }

    /// Set a key-value pair in the database or selected tree
    pub fn set_key(&self, key: &str, value: &str) -> Result<()> {
        if let Some(tree_name) = &self.selected_tree {
            let tree = self.get_tree(tree_name)?;
            tree.insert(key.as_bytes(), value.as_bytes())?;
            tree.flush()?;
        } else {
            self.db.insert(key.as_bytes(), value.as_bytes())?;
            self.db.flush()?;
        }
        Ok(())
    }

    /// Delete a key from the database or selected tree
    pub fn delete_key(&self, key: &str) -> Result<bool> {
        let existed = if let Some(tree_name) = &self.selected_tree {
            let tree = self.get_tree(tree_name)?;
            let existed = tree.remove(key.as_bytes())?.is_some();
            tree.flush()?;
            existed
        } else {
            let existed = self.db.remove(key.as_bytes())?.is_some();
            self.db.flush()?;
            existed
        };
        Ok(existed)
    }

    /// Check if the database is writable
    #[must_use]
    pub fn is_writable(&self) -> bool {
        // Try a test operation to check if the database is writable
        if let Some(tree_name) = &self.selected_tree {
            if let Ok(tree) = self.get_tree(tree_name) {
                match tree.insert(b"__sledoview_test__", b"test") {
                    Ok(_) => {
                        let _ = tree.remove(b"__sledoview_test__");
                        let _ = tree.flush();
                        true
                    }
                    Err(_) => false,
                }
            } else {
                false
            }
        } else {
            match self.db.insert(b"__sledoview_test__", b"test") {
                Ok(_) => {
                    let _ = self.db.remove(b"__sledoview_test__");
                    let _ = self.db.flush();
                    true
                }
                Err(_) => false,
            }
        }
    }

    /// List all tree names, optionally filtered by pattern
    pub fn list_trees(&self, pattern: &str, is_regex: bool) -> Result<Vec<String>, SledoViewError> {
        let regex = create_regex(pattern, is_regex)?;
        let mut tree_names = Vec::new();

        // Get all tree names from the database
        let all_trees = self.db.tree_names();

        for tree_name_bytes in all_trees {
            let tree_name = String::from_utf8_lossy(&tree_name_bytes).to_string();

            // Skip the default tree (empty name or __sled__default)
            if tree_name.is_empty() || tree_name == "__sled__default" {
                continue;
            }

            if regex.is_match(&tree_name) {
                tree_names.push(tree_name);
            }
        }

        tree_names.sort();
        Ok(tree_names)
    }

    /// Select a tree to work with
    pub fn select_tree(&mut self, tree_name: &str) -> Result<()> {
        // Verify the tree exists by trying to open it
        let _ = self.get_tree(tree_name)?;
        self.selected_tree = Some(tree_name.to_string());
        Ok(())
    }

    /// Unselect the current tree
    #[allow(clippy::unnecessary_wraps)] // avoid breaking public API
    pub fn unselect_tree(&mut self) -> Result<bool> {
        let was_selected = self.selected_tree.is_some();
        self.selected_tree = None;
        Ok(was_selected)
    }

    /// Get the currently selected tree name
    #[must_use]
    pub fn get_selected_tree(&self) -> Option<&String> {
        self.selected_tree.as_ref()
    }

    /// Get a tree by name
    fn get_tree(&self, name: &str) -> Result<Tree> {
        self.db.open_tree(name.as_bytes()).map_err(|e| {
            SledoViewError::TreeOperation {
                message: format!("Failed to open tree '{name}': {e}"),
            }
            .into()
        })
    }
}

#[derive(Debug)]
pub struct KeyInfo {
    /// Full key string: UTF-8 decoded for text keys, complete uppercase hex for binary keys.
    pub key: String,
    pub value: String,
    pub size: usize,
    pub is_utf8: bool,
}

#[derive(Debug)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
}

/// Format raw key bytes for display.
///
/// - Valid UTF-8: returned as the decoded string.
/// - Binary / invalid UTF-8: displayed as uppercase hex.
///   If the hex representation exceeds 32 characters it is shortened to
///   `XXXXXXXX....XXXXXXXX` (first 8 hex digits, four dots, last 8 hex digits).
pub fn format_key_bytes(bytes: &[u8]) -> String {
    if let Ok(s) = std::str::from_utf8(bytes) {
        return s.to_string();
    }
    let hex: String = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    if hex.len() > 32 {
        format!("{}....{}", &hex[..8], &hex[hex.len() - 8..])
    } else {
        hex
    }
}

/// Like [`format_key_bytes`] but never truncates — always returns the full
/// UTF-8 string or the complete uppercase hex representation.
pub fn format_key_bytes_full(bytes: &[u8]) -> String {
    if let Ok(s) = std::str::from_utf8(bytes) {
        return s.to_string();
    }
    bytes.iter().map(|b| format!("{:02X}", b)).collect()
}

/// Returns `true` when a sled error indicates the database lock is held by
/// another process.  Mirrors the logic in `validator::is_lock_error`.
fn is_sled_lock_error(err: &sled::Error) -> bool {
    if let sled::Error::Io(io_err) = err {
        match io_err.kind() {
            std::io::ErrorKind::PermissionDenied | std::io::ErrorKind::WouldBlock => return true,
            std::io::ErrorKind::Other => {
                if let Some(os_code) = io_err.raw_os_error() {
                    if os_code == 32 || os_code == 33 {
                        return true;
                    }
                }
            }
            _ => {}
        }
        if io_err.to_string().to_lowercase().contains("lock") {
            return true;
        }
    }
    err.to_string().to_lowercase().contains("lock")
}

fn glob_to_regex(pattern: &str) -> String {
    let mut regex = String::new();
    regex.push('^');

    for ch in pattern.chars() {
        match ch {
            '*' => regex.push_str(".*"),
            '?' => regex.push('.'),
            '[' => regex.push('['),
            ']' => regex.push(']'),
            '\\' => regex.push_str("\\\\"),
            '^' => regex.push_str("\\^"),
            '$' => regex.push_str("\\$"),
            '.' => regex.push_str("\\."),
            '|' => regex.push_str("\\|"),
            '+' => regex.push_str("\\+"),
            '(' => regex.push_str("\\("),
            ')' => regex.push_str("\\)"),
            '{' => regex.push_str("\\{"),
            '}' => regex.push_str("\\}"),
            c => regex.push(c),
        }
    }

    regex.push('$');
    regex
}

fn create_regex(pattern: &str, is_regex: bool) -> Result<Regex, SledoViewError> {
    if is_regex {
        Regex::new(pattern).map_err(|_| SledoViewError::InvalidRegex {
            pattern: pattern.to_string(),
        })
    } else {
        // Convert glob pattern to regex
        let regex_pattern = glob_to_regex(pattern);
        Regex::new(&regex_pattern).map_err(|_| SledoViewError::InvalidRegex {
            pattern: pattern.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_db() -> TempDir {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
        {
            let db = sled::open(temp_dir.path()).expect("Failed to create test database");
            db.insert(b"test_key", b"test_value").unwrap();
            db.insert(b"another_key", b"another_value").unwrap();
            db.flush().unwrap();
        }
        temp_dir
    }

    #[test]
    fn test_glob_to_regex() {
        assert_eq!(glob_to_regex("*"), "^.*$");
        assert_eq!(glob_to_regex("test*"), "^test.*$");
        assert_eq!(glob_to_regex("*test"), "^.*test$");
        assert_eq!(glob_to_regex("test?"), "^test.$");
        assert_eq!(glob_to_regex("test.txt"), "^test\\.txt$");
    }

    #[test]
    fn test_sled_viewer_new() {
        let temp_dir = create_test_db();
        let viewer = SledViewer::new(temp_dir.path());
        assert!(viewer.is_ok());
    }

    #[test]
    fn test_key_info_debug() {
        let info = KeyInfo {
            key: "test".to_string(),
            value: "value".to_string(),
            size: 5,
            is_utf8: true,
        };
        let debug_str = format!("{:?}", info);
        assert!(debug_str.contains("test"));
        assert!(debug_str.contains("value"));
    }

    #[test]
    fn test_key_value_pair_debug() {
        let pair = KeyValuePair {
            key: "test".to_string(),
            value: "value".to_string(),
        };
        let debug_str = format!("{:?}", pair);
        assert!(debug_str.contains("test"));
        assert!(debug_str.contains("value"));
    }

    #[test]
    fn test_set_key() {
        let temp_dir = create_test_db();
        let viewer = SledViewer::new(temp_dir.path()).unwrap();

        // Test setting a new key
        assert!(viewer.set_key("new_key", "new_value").is_ok());

        // Verify the key was set
        let info = viewer.get_key("new_key").unwrap();
        assert_eq!(info.key, "new_key");
        assert_eq!(info.value, "new_value");

        // Test updating an existing key
        assert!(viewer.set_key("new_key", "updated_value").is_ok());
        let info = viewer.get_key("new_key").unwrap();
        assert_eq!(info.value, "updated_value");
    }

    #[test]
    fn test_delete_key() {
        let temp_dir = create_test_db();
        let viewer = SledViewer::new(temp_dir.path()).unwrap();

        // First, set a key
        viewer.set_key("test_delete", "value").unwrap();
        assert!(viewer.get_key("test_delete").is_ok());

        // Test deleting an existing key
        let existed = viewer.delete_key("test_delete").unwrap();
        assert!(existed);

        // Verify the key was deleted
        assert!(viewer.get_key("test_delete").is_err());

        // Test deleting a non-existent key
        let existed = viewer.delete_key("non_existent").unwrap();
        assert!(!existed);
    }

    #[test]
    fn test_is_writable() {
        let temp_dir = create_test_db();
        let viewer = SledViewer::new(temp_dir.path()).unwrap();

        // Database should be writable in tests
        assert!(viewer.is_writable());
    }

    #[test]
    fn test_set_with_special_characters() {
        let temp_dir = create_test_db();
        let viewer = SledViewer::new(temp_dir.path()).unwrap();

        // Test with spaces and special characters
        assert!(viewer
            .set_key("key with spaces", "value with spaces")
            .is_ok());
        let info = viewer.get_key("key with spaces").unwrap();
        assert_eq!(info.value, "value with spaces");

        // Test with quotes and escapes
        assert!(viewer.set_key("quote_key", "value with \"quotes\"").is_ok());
        let info = viewer.get_key("quote_key").unwrap();
        assert_eq!(info.value, "value with \"quotes\"");
    }

    #[test]
    fn test_create_tree_db() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let db = sled::open(temp_dir.path())?;

        // Create some trees with data
        let tree1 = db.open_tree(b"tree1")?;
        tree1.insert(b"key1", b"value1")?;
        tree1.insert(b"key2", b"value2")?;

        let tree2 = db.open_tree(b"tree2")?;
        tree2.insert(b"keyA", b"valueA")?;
        tree2.insert(b"keyB", b"valueB")?;

        // Add some data to default tree
        db.insert(b"default_key", b"default_value")?;

        db.flush()?;
        tree1.flush()?;
        tree2.flush()?;

        Ok(())
    }

    #[test]
    fn test_list_trees() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test_list_trees");
        {
            let db = sled::open(&db_path).unwrap();

            // Create some trees
            let _tree1 = db.open_tree(b"settings").unwrap();
            let _tree2 = db.open_tree(b"sessions").unwrap();
            let _tree3 = db.open_tree(b"cache").unwrap();
            let _tree4 = db.open_tree(b"my_tree_1").unwrap();
            let _tree5 = db.open_tree(b"my_tree_2").unwrap();

            db.flush().unwrap();
            // DB is dropped here, closing all handles
        }

        let viewer = SledViewer::new(&db_path).unwrap();

        // Test listing all trees
        let trees = viewer.list_trees("*", false).unwrap();
        assert!(trees.contains(&"settings".to_string()));
        assert!(trees.contains(&"sessions".to_string()));
        assert!(trees.contains(&"cache".to_string()));
        assert!(trees.contains(&"my_tree_1".to_string()));
        assert!(trees.contains(&"my_tree_2".to_string()));

        // Test pattern matching
        let trees = viewer.list_trees("my_tree_*", false).unwrap();
        assert_eq!(trees.len(), 2);
        assert!(trees.contains(&"my_tree_1".to_string()));
        assert!(trees.contains(&"my_tree_2".to_string()));

        // Test regex matching
        let trees = viewer.list_trees(r"my_tree_\d+", true).unwrap();
        assert_eq!(trees.len(), 2);
        assert!(trees.contains(&"my_tree_1".to_string()));
        assert!(trees.contains(&"my_tree_2".to_string()));

        // Test no matches
        let trees = viewer.list_trees("nonexistent_*", false).unwrap();
        assert!(trees.is_empty());
    }

    #[test]
    fn test_tree_selection() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test_tree_selection");
        {
            let db = sled::open(&db_path).unwrap();

            // Create a tree with data
            let tree = db.open_tree(b"test_tree").unwrap();
            tree.insert(b"tree_key", b"tree_value").unwrap();
            tree.flush().unwrap();

            // Add data to default tree
            db.insert(b"default_key", b"default_value").unwrap();
            db.flush().unwrap();
            // DB is dropped here, closing all handles
        }

        let mut viewer = SledViewer::new(&db_path).unwrap();

        // Initially no tree should be selected
        assert!(viewer.get_selected_tree().is_none());

        // Select a tree
        assert!(viewer.select_tree("test_tree").is_ok());
        assert_eq!(viewer.get_selected_tree().unwrap(), "test_tree");

        // Unselect tree
        let was_selected = viewer.unselect_tree().unwrap();
        assert!(was_selected);
        assert!(viewer.get_selected_tree().is_none());

        // Unselect when none selected
        let was_selected = viewer.unselect_tree().unwrap();
        assert!(!was_selected);
    }

    #[test]
    fn test_tree_operations_with_selection() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test_tree_operations");
        {
            let db = sled::open(&db_path).unwrap();

            // Create trees with data
            let tree1 = db.open_tree(b"tree1").unwrap();
            tree1.insert(b"key1", b"tree1_value1").unwrap();
            tree1.insert(b"key2", b"tree1_value2").unwrap();

            let tree2 = db.open_tree(b"tree2").unwrap();
            tree2.insert(b"key1", b"tree2_value1").unwrap();
            tree2.insert(b"keyA", b"tree2_valueA").unwrap();

            // Add data to default tree
            db.insert(b"key1", b"default_value1").unwrap();
            db.insert(b"default_key", b"default_value").unwrap();

            db.flush().unwrap();
            tree1.flush().unwrap();
            tree2.flush().unwrap();
            // DB is dropped here, closing all handles
        }

        let mut viewer = SledViewer::new(&db_path).unwrap();

        // Test operations on default tree
        let keys = viewer.list_keys("*", false).unwrap();
        assert!(keys.contains(&"key1".to_string()));
        assert!(keys.contains(&"default_key".to_string()));
        assert_eq!(keys.len(), 2);

        let info = viewer.get_key("key1").unwrap();
        assert_eq!(info.value, "default_value1");

        // Select tree1
        viewer.select_tree("tree1").unwrap();

        // Test operations on tree1
        let keys = viewer.list_keys("*", false).unwrap();
        assert!(keys.contains(&"key1".to_string()));
        assert!(keys.contains(&"key2".to_string()));
        assert_eq!(keys.len(), 2);

        let info = viewer.get_key("key1").unwrap();
        assert_eq!(info.value, "tree1_value1");

        // Test count
        assert_eq!(viewer.count().unwrap(), 2);

        // Select tree2
        viewer.select_tree("tree2").unwrap();

        let keys = viewer.list_keys("*", false).unwrap();
        assert!(keys.contains(&"key1".to_string()));
        assert!(keys.contains(&"keyA".to_string()));
        assert_eq!(keys.len(), 2);

        let info = viewer.get_key("key1").unwrap();
        assert_eq!(info.value, "tree2_value1");

        // Test count
        assert_eq!(viewer.count().unwrap(), 2);
    }

    #[test]
    fn test_tree_set_and_delete_operations() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test_tree_set_delete");
        {
            let db = sled::open(&db_path).unwrap();

            // Create a tree
            let _tree = db.open_tree(b"test_tree").unwrap();
            db.flush().unwrap();
            // DB is dropped here, closing all handles
        }

        let mut viewer = SledViewer::new(&db_path).unwrap();

        // Test set on default tree
        viewer.set_key("default_key", "default_value").unwrap();
        let info = viewer.get_key("default_key").unwrap();
        assert_eq!(info.value, "default_value");

        // Select tree and test set
        viewer.select_tree("test_tree").unwrap();
        viewer.set_key("tree_key", "tree_value").unwrap();
        let info = viewer.get_key("tree_key").unwrap();
        assert_eq!(info.value, "tree_value");

        // Key shouldn't exist in default tree
        viewer.unselect_tree().unwrap();
        assert!(viewer.get_key("tree_key").is_err());

        // But default key should still exist
        let info = viewer.get_key("default_key").unwrap();
        assert_eq!(info.value, "default_value");

        // Test delete on tree
        viewer.select_tree("test_tree").unwrap();
        let existed = viewer.delete_key("tree_key").unwrap();
        assert!(existed);
        assert!(viewer.get_key("tree_key").is_err());

        // Test delete non-existent key
        let existed = viewer.delete_key("non_existent").unwrap();
        assert!(!existed);
    }

    #[test]
    fn test_tree_search_operations() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test_tree_search");
        {
            let db = sled::open(&db_path).unwrap();

            // Create tree with data
            let tree = db.open_tree(b"search_tree").unwrap();
            tree.insert(b"user_001", b"John Doe").unwrap();
            tree.insert(b"user_002", b"Jane Smith").unwrap();
            tree.insert(b"admin_001", b"Admin User").unwrap();

            // Add data to default tree
            db.insert(b"user_001", b"Default John").unwrap();
            db.insert(b"config_001", b"Config Value").unwrap();

            db.flush().unwrap();
            tree.flush().unwrap();
            // DB is dropped here, closing all handles
        }

        let mut viewer = SledViewer::new(&db_path).unwrap();

        // Search in default tree
        let results = viewer.search_values("*John*", false).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].key, "user_001");
        assert_eq!(results[0].value, "Default John");

        // Select tree and search
        viewer.select_tree("search_tree").unwrap();
        let results = viewer.search_values("*John*", false).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].key, "user_001");
        assert_eq!(results[0].value, "John Doe");

        // Search with pattern
        let results = viewer.search_values("*Smith", false).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].value, "Jane Smith");

        // Search keys
        let keys = viewer.list_keys("user_*", false).unwrap();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"user_001".to_string()));
        assert!(keys.contains(&"user_002".to_string()));
    }

    #[test]
    fn test_tree_errors() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test_tree_errors");
        let mut viewer = SledViewer::new(&db_path).unwrap();

        // Test selecting non-existent tree should still work (sled creates it)
        assert!(viewer.select_tree("nonexistent_tree").is_ok());
        assert_eq!(viewer.get_selected_tree().unwrap(), "nonexistent_tree");

        // Test operations on empty tree
        let keys = viewer.list_keys("*", false).unwrap();
        assert!(keys.is_empty());

        let count = viewer.count().unwrap();
        assert_eq!(count, 0);
    }
}
