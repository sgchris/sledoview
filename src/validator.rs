use crate::db::SledViewer;
use crate::error::{Result, SledoViewError};
use std::path::Path;

pub struct DatabaseValidator<'a> {
    path: &'a Path,
}

impl<'a> DatabaseValidator<'a> {
    #[must_use]
    pub fn new(path: &'a Path) -> Self {
        Self { path }
    }

    #[allow(dead_code)]
    pub fn validate(&self) -> Result<()> {
        self.inspect_path()?;
        let _viewer = SledViewer::new(self.path)?;
        Ok(())
    }

    pub fn open(&self) -> Result<SledViewer> {
        self.inspect_path()?;
        SledViewer::new(self.path)
    }

    fn inspect_path(&self) -> Result<()> {
        if !self.path.exists() {
            return Err(SledoViewError::DatabaseNotFound {
                path: self.path.display().to_string(),
            });
        }

        if !self.path.is_dir() {
            return Err(SledoViewError::InvalidSledDatabase {
                path: self.path.display().to_string(),
            });
        }

        Ok(())
    }
}
