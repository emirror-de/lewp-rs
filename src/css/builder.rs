use {
    super::Css,
    crate::fh::{FileHierarchy, Level},
    std::path::PathBuf,
};

/// Helps creating a Css instance.
pub struct CssBuilder {
    fh: FileHierarchy,
    excluded_files: Vec<PathBuf>,
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
            excluded_files: vec![],
        }
    }

    /// Sets the file hierarchy.
    pub fn file_hierarchy(&mut self, fh: FileHierarchy) {
        self.fh = fh;
    }

    /// Sets the excluded files.
    pub fn excluded_files(&mut self, files: Vec<PathBuf>) {
        self.excluded_files = files;
    }

    /// Creates the Css instance.
    pub fn build(self) -> Css {
        Css {
            fh: self.fh,
            excluded_files: self.excluded_files,
            id: self.id,
            level: self.level,
        }
    }
}
