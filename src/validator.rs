use crate::db::SledViewer;
use crate::error::{Result, SledoViewError};
use colored::*;
use std::fs;
use std::path::Path;

pub struct DatabaseValidator<'a> {
    path: &'a Path,
}

impl<'a> DatabaseValidator<'a> {
    #[must_use]
    pub fn new(path: &'a Path) -> Self {
        Self { path }
    }

    pub fn validate(&self) -> Result<()> {
        println!("{}", "Validating database...".yellow());

        self.check_file_exists()?;
        self.check_file_readable()?;
        self.check_is_directory()?;
        self.check_sled_structure()?;

        println!(
            "{} {}",
            "✓".bright_green(),
            "Database validation passed".green()
        );
        Ok(())
    }

    pub fn open(&self) -> Result<SledViewer> {
        self.validate()?;
        SledViewer::new(self.path)
    }

    fn check_file_exists(&self) -> Result<()> {
        if !self.path.exists() {
            return Err(SledoViewError::DatabaseNotFound {
                path: self.path.display().to_string(),
            });
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
                        }),
                    }
                } else {
                    Ok(())
                }
            }
            Err(_) => Err(SledoViewError::PermissionDenied {
                path: self.path.display().to_string(),
            }),
        }
    }

    fn check_is_directory(&self) -> Result<()> {
        if !self.path.is_dir() {
            return Err(SledoViewError::InvalidSledDatabase {
                path: self.path.display().to_string(),
            });
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
                });
            }
        }

        Ok(())
    }
}
