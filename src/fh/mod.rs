//! Defines the file hierarchy of [lewp](crate).

use std::path::{Path, PathBuf};

mod builder;

pub use builder::FileHierarchyBuilder;

/// Possible file types that the file hierarchy is covering.
pub enum FileType {
    /// A CSS file with `.css` extension.
    CSS,
    /// A JavaScript file with `.js` extension.
    JavaScript,
}

/// The file hierarchy level.
#[derive(Debug, Clone)]
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
    pub fn folder(&self, id: &str, file_type: FileType, level: Level) -> PathBuf {
        let mut path = self.base_directory.clone();
        path.push(self.level(level));
        path.push(id);
        path.push(self.extension(file_type));
        path
    }

    /// Collects all filenames recursively in the given subfolder. `subfolder`
    /// is referenced to the base directory given in the FileHierarchy instance.
    /// Parts containing `../` are removed before processing.
    pub fn collect_filenames(&self, subfolder: &str) -> Vec<PathBuf> {
        let isolated_subfolder = self.isolate_path(subfolder);
        let subfolder = self.base_directory.join(Path::new(&isolated_subfolder));
        if !subfolder.is_dir() {
            return vec![];
        }
        let mut filenames = vec![];
        for entry in walkdir::WalkDir::new(&subfolder) {
            let entry = match entry {
                Ok(v) => v.into_path(),
                Err(_) => continue,
            };
            if entry.is_dir() {
                continue;
            }
            let entry = match self.remove_base_dir(&subfolder, &entry) {
                Ok(p) => p,
                Err(msg) => {
                    log::error!("{}", msg);
                    continue;
                }
            };
            filenames.push(entry)
        }
        filenames
    }

    fn remove_base_dir(&self, base_dir: &Path, input_path: &Path) -> Result<PathBuf, String> {
        match pathdiff::diff_paths(input_path, base_dir) {
            Some(p) => Ok(p),
            None => match input_path.to_str() {
                Some(v) => Err(format!("Could not remove base dir of {}", v)),
                None => Err("Could not remove base dir!".to_string()),
            },
        }
    }

    /// Returns the correct extension for the given file type.
    pub(crate) fn extension(&self, file_type: FileType) -> &str {
        match file_type {
            FileType::CSS => "css",
            FileType::JavaScript => "js",
        }
    }

    /// Returns the correct level part.
    fn level(&self, level: Level) -> &str {
        match level {
            Level::Page => "pages",
            Level::Module => "modules",
        }
    }

    /// Removes `../` from the given string to isolate the filepath to a base
    /// directory.
    fn isolate_path(&self, path: &str) -> String {
        let s = String::from(path);
        let mut s = s.split('/').collect::<Vec<&str>>();
        s.retain(|&e| !e.contains(".."));
        s.join("/")
    }
}

impl Default for FileHierarchy {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn folder_name_generation() {
    let fh = FileHierarchy::new();
    assert_eq!(
        "./modules/module-id/css",
        fh.folder("module-id", FileType::CSS, Level::Module)
            .to_str()
            .unwrap()
    );
    assert_eq!(
        "./pages/hello-world/js",
        fh.folder("hello-world", FileType::JavaScript, Level::Page)
            .to_str()
            .unwrap()
    );
}

#[test]
fn isolate_file_paths() {
    let fh = FileHierarchyBuilder::new().build();
    let breakout = "../something";
    let isolated = fh.isolate_path(breakout);
    assert_eq!(isolated, "something");
    let non_breakout = "something/subfolder";
    let isolated = fh.isolate_path(non_breakout);
    assert_eq!(isolated, "something/subfolder");
}

#[test]
fn collect_filenames() {
    let fh = FileHierarchyBuilder::new()
        .base_directory(PathBuf::from("testfiles"))
        .build();
    let mut filenames = fh.collect_filenames("");
    let mut reference = vec![
        PathBuf::from("modules/hello-world/css/primary.css"),
        PathBuf::from("modules/hello-world/css/secondary.css"),
    ];
    assert_eq!(filenames.sort(), reference.sort());
}
