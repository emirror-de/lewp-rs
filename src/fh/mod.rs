//! Defines the file hierarchy of [lewp](crate).

use std::path::PathBuf;

/// Possible file types that the file hierarchy is covering.
pub enum FileType {
    /// A CSS file with `.css` extension.
    CSS,
    /// A JavaScript file with `.js` extension.
    JavaScript,
}

/// The file hierarchy level.
pub enum Level {
    /// The module level.
    Module,
    /// The page level.
    Page,
}

/// File hierarchy instance, handles file path generation.
pub struct FileHierarchy {
    base_directory: PathBuf,
}

impl FileHierarchy {
    /// Creates a new file hierarchy instance.
    pub fn new() -> Self {
        Self {
            base_directory: PathBuf::from("."),
        }
    }

    /// Generates the folder path according to the file hierarchy. The folder
    /// that contains the `file_type` always corresponds to the extension of the
    /// files contained.
    pub fn folder(&self, file_type: FileType, level: Level) -> PathBuf {
        let mut path = self.base_directory.clone();
        path.push(self.level(&level));
        path.push(self.extension(&file_type));
        path
    }

    /// Returns the correct extension for the given file type.
    fn extension(&self, file_type: &FileType) -> &str {
        match file_type {
            FileType::CSS => "css",
            FileType::JavaScript => "js",
        }
    }

    /// Returns the correct level part.
    fn level(&self, level: &Level) -> &str {
        match level {
            Level::Page => "pages",
            Level::Module => "modules",
        }
    }
}

impl Default for FileHierarchy {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn folder_creation() {
    let fh = FileHierarchy::new();
    assert_eq!(
        "./modules/css",
        fh.folder(FileType::CSS, Level::Module).to_str().unwrap()
    );
    assert_eq!(
        "./pages/js",
        fh.folder(FileType::JavaScript, Level::Page)
            .to_str()
            .unwrap()
    );
}
