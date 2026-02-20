use crate::error::SledoViewError;
use anyhow::Result;
use colored::*;
use std::fs;
use std::path::Path;

/// Returns `true` when a sled error looks like a lock/contention error.
///
/// Sled serialises database access with a lock file. When another process
/// already holds that lock the underlying OS error varies by platform:
/// - POSIX: `EWOULDBLOCK` / `EAGAIN` → `ErrorKind::WouldBlock`
/// - Windows: `ERROR_LOCK_VIOLATION` (33) or `ERROR_SHARING_VIOLATION` (32)
///   → `ErrorKind::Other`
/// - Fallback: the error message contains the word "lock".
fn is_lock_error(err: &sled::Error) -> bool {
    if let sled::Error::Io(io_err) = err {
        match io_err.kind() {
            std::io::ErrorKind::PermissionDenied | std::io::ErrorKind::WouldBlock => return true,
            std::io::ErrorKind::Other => {
                // Windows ERROR_LOCK_VIOLATION = 33, ERROR_SHARING_VIOLATION = 32
                if let Some(os_code) = io_err.raw_os_error() {
                    if os_code == 32 || os_code == 33 {
                        return true;
                    }
                }
            }
            _ => {}
        }
        // Fallback: inspect the message text
        if io_err.to_string().to_lowercase().contains("lock") {
            return true;
        }
    }
    // Also check the top-level sled error message
    err.to_string().to_lowercase().contains("lock")
}

pub struct DatabaseValidator<'a> {
    path: &'a Path,
}

impl<'a> DatabaseValidator<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self { path }
    }

    pub fn validate(&self) -> Result<()> {
        println!("{}", "Validating database...".yellow());

        self.check_file_exists()?;
        self.check_file_readable()?;
        self.check_is_directory()?;
        self.check_sled_structure()?;
        self.check_not_locked()?;

        println!(
            "{} {}",
            "✓".bright_green(),
            "Database validation passed".green()
        );
        Ok(())
    }

    fn check_file_exists(&self) -> Result<()> {
        if !self.path.exists() {
            return Err(SledoViewError::DatabaseNotFound {
                path: self.path.display().to_string(),
            }
            .into());
        }
        Ok(())
    }

    fn check_file_readable(&self) -> Result<()> {
        match fs::metadata(self.path) {
            Ok(metadata) => {
                if metadata.permissions().readonly() {
                    // On Windows, readonly doesn't necessarily mean we can't read
                    // Let's try to open the directory for reading
                    match fs::read_dir(self.path) {
                        Ok(_) => Ok(()),
                        Err(_) => Err(SledoViewError::DatabaseNotReadable {
                            path: self.path.display().to_string(),
                        }
                        .into()),
                    }
                } else {
                    Ok(())
                }
            }
            Err(_) => Err(SledoViewError::PermissionDenied {
                path: self.path.display().to_string(),
            }
            .into()),
        }
    }

    fn check_is_directory(&self) -> Result<()> {
        if !self.path.is_dir() {
            return Err(SledoViewError::InvalidSledDatabase {
                path: self.path.display().to_string(),
            }
            .into());
        }
        Ok(())
    }

    fn check_sled_structure(&self) -> Result<()> {
        // SLED databases are directories with specific files
        // Check for common SLED files like "conf" and "db"
        let conf_path = self.path.join("conf");
        let db_path = self.path.join("db");

        if !conf_path.exists() && !db_path.exists() {
            // Try to detect if it's a SLED database by looking for any typical files
            let entries = fs::read_dir(self.path)?;
            let mut has_sled_files = false;

            for entry in entries.flatten() {
                let file_name = entry.file_name();
                let name = file_name.to_string_lossy();
                // SLED creates files with numeric names or "conf"
                if name == "conf" || name.chars().all(|c| c.is_ascii_digit()) {
                    has_sled_files = true;
                    break;
                }
            }

            if !has_sled_files {
                return Err(SledoViewError::InvalidSledDatabase {
                    path: self.path.display().to_string(),
                }
                .into());
            }
        }

        Ok(())
    }

    fn check_not_locked(&self) -> Result<()> {
        // Try to open the database to check if it's locked
        match sled::open(self.path) {
            Ok(_) => Ok(()),
            Err(ref e) if is_lock_error(e) => Err(SledoViewError::DatabaseLocked {
                path: self.path.display().to_string(),
            }
            .into()),
            Err(e) => Err(SledoViewError::DatabaseOperation {
                message: format!("Failed to open database: {e}"),
            }
            .into()),
        }
    }
}
