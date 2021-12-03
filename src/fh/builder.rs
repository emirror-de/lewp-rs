use {crate::fh::FileHierarchy, std::path::PathBuf};

/// Helper struct for creating a FileHierarchy instance.
pub struct FileHierarchyBuilder {
    base_directory: PathBuf,
}

impl FileHierarchyBuilder {
    /// Returns the default instance.
    pub fn new() -> Self {
        Self {
            base_directory: PathBuf::from("."),
        }
    }

    /// Sets the base directory of the file hierarchy.
    pub fn base_directory(mut self, base_directory: PathBuf) -> Self {
        self.base_directory = base_directory;
        self
    }

    /// Builds the file hierarchy instance.
    pub fn build(self) -> FileHierarchy {
        FileHierarchy {
            base_directory: self.base_directory,
        }
    }
}

impl Default for FileHierarchyBuilder {
    fn default() -> Self {
        Self::new()
    }
}
