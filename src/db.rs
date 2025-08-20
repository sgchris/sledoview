use crate::error::SledoViewError;
use anyhow::Result;
use regex::Regex;
use sled::Db;
use std::path::Path;

pub struct SledViewer {
    db: Db,
}

impl SledViewer {
    pub fn new(path: &Path) -> Result<Self> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    pub fn count(&self) -> Result<usize> {
        Ok(self.db.len())
    }

    pub fn list_keys(&self, pattern: &str, is_regex: bool) -> Result<Vec<String>> {
        let mut keys = Vec::new();

        if is_regex {
            let regex = Regex::new(pattern).map_err(|_| SledoViewError::InvalidRegex {
                pattern: pattern.to_string(),
            })?;

            for result in self.db.iter() {
                let (key, _) = result?;
                let key_str = String::from_utf8_lossy(&key);
                if regex.is_match(&key_str) {
                    keys.push(key_str.to_string());
                }
            }
        } else {
            // Convert glob pattern to regex
            let regex_pattern = glob_to_regex(pattern);
            let regex = Regex::new(&regex_pattern).map_err(|_| SledoViewError::InvalidRegex {
                pattern: pattern.to_string(),
            })?;

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

    pub fn get_key(&self, key: &str) -> Result<KeyInfo> {
        let key_bytes = key.as_bytes();

        match self.db.get(key_bytes)? {
            Some(value) => {
                let value_str = String::from_utf8_lossy(&value);
                let size = value.len();

                Ok(KeyInfo {
                    key: key.to_string(),
                    value: value_str.to_string(),
                    size,
                    is_utf8: String::from_utf8(value.to_vec()).is_ok(),
                })
            }
            None => Err(SledoViewError::KeyNotFound {
                key: key.to_string(),
            }
            .into()),
        }
    }

    pub fn search_values(&self, pattern: &str, is_regex: bool) -> Result<Vec<KeyValuePair>> {
        let mut results = Vec::new();

        if is_regex {
            let regex = Regex::new(pattern).map_err(|_| SledoViewError::InvalidRegex {
                pattern: pattern.to_string(),
            })?;

            for result in self.db.iter() {
                let (key, value) = result?;
                let value_str = String::from_utf8_lossy(&value);
                if regex.is_match(&value_str) {
                    results.push(KeyValuePair {
                        key: String::from_utf8_lossy(&key).to_string(),
                        value: value_str.to_string(),
                    });
                }
            }
        } else {
            // Convert glob pattern to regex
            let regex_pattern = glob_to_regex(pattern);
            let regex = Regex::new(&regex_pattern).map_err(|_| SledoViewError::InvalidRegex {
                pattern: pattern.to_string(),
            })?;

            for result in self.db.iter() {
                let (key, value) = result?;
                let value_str = String::from_utf8_lossy(&value);
                if regex.is_match(&value_str) {
                    results.push(KeyValuePair {
                        key: String::from_utf8_lossy(&key).to_string(),
                        value: value_str.to_string(),
                    });
                }
            }
        }

        results.sort_by(|a, b| a.key.cmp(&b.key));
        Ok(results)
    }
}

#[derive(Debug)]
pub struct KeyInfo {
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
}
