use {crate::fh::FileHierarchy, std::path::PathBuf};

/// Helper struct for creating a FileHierarchy instance.
pub struct FileHierarchyBuilder {
    mountpoint: PathBuf,
}

impl FileHierarchyBuilder {
    /// Returns the default instance.
    pub fn new() -> Self {
        Self {
            mountpoint: PathBuf::from("."),
        }
    }

    /// Sets the base directory of the file hierarchy.
    pub fn mountpoint(mut self, mountpoint: PathBuf) -> Self {
        self.mountpoint = mountpoint;
        self
    }

    /// Builds the file hierarchy instance.
    pub fn build(self) -> FileHierarchy {
        FileHierarchy {
            mountpoint: self.mountpoint,
        }
    }
}

impl Default for FileHierarchyBuilder {
    fn default() -> Self {
        Self::new()
    }
}
