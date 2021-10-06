use {
    super::Css,
    crate::fh::{FileHierarchy, Level},
    std::path::PathBuf,
};

/// Helps creating a Css instance.
pub struct CssBuilder {
    fh: FileHierarchy,
    exclude_files: Vec<PathBuf>,
    id: String,
    level: Level,
}

impl CssBuilder {
    /// Creates a new instance.
    pub fn new(id: &str, level: Level) -> Self {
        Self {
            id: id.to_string(),
            level,
            fh: FileHierarchy::new(),
            exclude_files: vec![],
        }
    }

    /// Sets the file hierarchy.
    pub fn file_hierarchy(mut self, fh: FileHierarchy) -> Self {
        self.fh = fh;
        self
    }

    /// Sets the excluded files.
    pub fn exclude_files(mut self, files: Vec<PathBuf>) -> Self {
        self.exclude_files = files;
        self
    }

    /// Creates the Css instance.
    pub fn build(self) -> Css {
        Css {
            fh: self.fh,
            exclude_files: self.exclude_files,
            id: self.id,
            level: self.level,
        }
    }
}
