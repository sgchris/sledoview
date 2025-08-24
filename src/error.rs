use thiserror::Error;

#[derive(Error, Debug)]
pub enum SledoViewError {
    #[error("Database file not found: {path}")]
    DatabaseNotFound { path: String },

    #[error("Database file is not readable: {path}")]
    DatabaseNotReadable { path: String },

    #[error("File is not a SLED database: {path}")]
    InvalidSledDatabase { path: String },

    #[error("Database is locked by another process: {path}")]
    DatabaseLocked { path: String },

    #[error("Permission denied accessing database: {path}")]
    PermissionDenied { path: String },

    #[error("Invalid regex pattern: {pattern}")]
    InvalidRegex { pattern: String },

    #[error("Key not found: {key}")]
    KeyNotFound { key: String },

    #[error("Database operation failed: {message}")]
    DatabaseOperation { message: String },

    #[error("Tree operation failed: {message}")]
    TreeOperation { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("SLED error: {0}")]
    Sled(#[from] sled::Error),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = SledoViewError::DatabaseNotFound {
            path: "/test/path".to_string(),
        };
        assert_eq!(err.to_string(), "Database file not found: /test/path");

        let err = SledoViewError::InvalidRegex {
            pattern: "[invalid".to_string(),
        };
        assert_eq!(err.to_string(), "Invalid regex pattern: [invalid");

        let err = SledoViewError::KeyNotFound {
            key: "missing_key".to_string(),
        };
        assert_eq!(err.to_string(), "Key not found: missing_key");
    }

    #[test]
    fn test_error_debug() {
        let err = SledoViewError::DatabaseLocked {
            path: "/test/path".to_string(),
        };
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("DatabaseLocked"));
        assert!(debug_str.contains("/test/path"));
    }
}
