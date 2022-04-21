//! Defines the file hierarchy of [lewp](crate).

use {
    crate::{LewpError, LewpErrorKind},
    std::{
        path::{Path, PathBuf},
        sync::Arc,
    },
};

mod builder;
mod component;
mod level;

pub use {
    builder::FileHierarchyBuilder,
    component::{Component, ComponentInformation, ComponentType, ResourceType},
    level::Level,
};

/// File hierarchy definition, handles file path generation.
pub struct FileHierarchy {
    mountpoint: PathBuf,
}

impl FileHierarchy {
    /// Creates a new file hierarchy instance.
    ///
    /// The mounpoint is set to "."
    pub fn new() -> Self {
        Self {
            mountpoint: PathBuf::from("."),
        }
    }

    /// Returns the directory where the file hierarchy is mounted.
    pub fn mountpoint(&self) -> PathBuf {
        self.mountpoint.clone()
    }

    /// Generates the folder path according to the file hierarchy. The folder
    /// that contains the `file_type` always corresponds to the extension of the
    /// files contained.
    pub fn folder<COMP: Component>(&self, component: &COMP) -> PathBuf {
        let mut path = self.mountpoint.clone();
        path.push(component.level().to_string());
        path.push(component.id().to_string());
        path.push(component.kind().to_string());
        path
    }

    /// Collects all filenames in the given component. The resulting
    /// vector contains the filepath including the mountpoint of the FileHierarchy.
    /// This function is not recursive.
    pub fn get_file_list<COMP: Component>(
        &self,
        component: &COMP,
    ) -> Result<Vec<PathBuf>, LewpError> {
        let subfolder = self
            .mountpoint
            .join(Path::new(&component.level().to_string()))
            .join(Path::new(&component.id().to_string()))
            .join(Path::new(&component.kind().to_string()));
        if !subfolder.is_dir() {
            return Err(LewpError {
                kind: LewpErrorKind::FileHierarchy,
                message: format!(
                    "Given input is not a folder: {}",
                    subfolder.display()
                ),
                source_component: component.component_information(),
            });
        }
        let mut filenames = vec![];
        for entry in walkdir::WalkDir::new(&subfolder) {
            let entry = match entry {
                Ok(v) => v.into_path(),
                Err(msg) => {
                    return Err(LewpError {
                        kind: LewpErrorKind::FileHierarchy,
                        message: msg.to_string(),
                        source_component: component.component_information(),
                    });
                }
            };
            if entry.is_dir() {
                // skip folders because we only want to get the files in the list
                continue;
            }
            filenames.push(entry);
        }
        filenames.sort();
        Ok(filenames)
    }

    fn remove_mountpoint(
        mountpoint: &Path,
        input_path: &Path,
    ) -> Result<PathBuf, String> {
        match pathdiff::diff_paths(input_path, mountpoint) {
            Some(p) => Ok(p),
            None => match input_path.to_str() {
                Some(v) => Err(format!("Could not remove base dir of {}", v)),
                None => Err("Could not remove base dir!".to_string()),
            },
        }
    }

    /// Gets a list of the component ids available for this [ComponentType] on the
    /// given [Level].
    pub fn collect_component_ids(
        &self,
        kind: ComponentType,
        level: Level,
    ) -> Result<Vec<String>, LewpError> {
        let mut v = vec![];
        // create a pattern to search for
        let pattern = PathBuf::from(&level.to_string())
            .join("*")
            .join(&kind.to_string());
        log::trace!("pattern: {:#?}", pattern);
        // combine it with the mountpoint
        let filepath = self.mountpoint().join(pattern);
        let filepath = match filepath.to_str() {
            Some(f) => f,
            None => {
                return Err(LewpError {
                    kind: LewpErrorKind::FileHierarchy,
                    message: String::from(
                        "Error converting filepath to string!",
                    ),
                    source_component: Arc::new(ComponentInformation::core(
                        "get_component_ids",
                    )),
                })
            }
        };
        log::trace!("filepath: {:#?}", filepath.to_string());
        // glob it!
        let glob_paths = match glob::glob(&filepath) {
            Ok(paths) => paths,
            Err(e) => {
                return Err(LewpError {
                    kind: LewpErrorKind::FileHierarchy,
                    message: format!("Error during glob call: {}", e),
                    source_component: Arc::new(ComponentInformation::core(
                        "get_component_ids",
                    )),
                })
            }
        };
        // iterate through paths
        for path in glob_paths {
            match path {
                Ok(p) => {
                    if let Some(ext) = kind.extension() {
                        // an extension is available so count the number of files
                        let count = walkdir::WalkDir::new(&p)
                            .follow_links(true)
                            .into_iter()
                            .filter_entry(|e| {
                                if e.file_type().is_dir() {
                                    return true;
                                }
                                let depth = e.depth() == 1;
                                let extension = e
                                    .file_name()
                                    .to_str()
                                    .map(|s| s.ends_with(&format!(".{}", ext)))
                                    .unwrap_or(false);
                                depth && extension
                            })
                            .count();
                        // skip path if there are no files
                        // if there is only one entry, it is the directory
                        // itself where we are iterating over
                        if count == 1 {
                            continue;
                        }
                    }
                    let p = self.extract_component_ids_from_pathbuf(&p)?;
                    v.push(p);
                }
                Err(e) => {
                    return Err(LewpError {
                        kind: LewpErrorKind::FileHierarchy,
                        message: format!(
                            "Error during glob call: {}",
                            e.into_error()
                        ),
                        source_component: Arc::new(ComponentInformation::core(
                            "get_component_ids",
                        )),
                    })
                }
            }
        }
        log::trace!("Found component ids:\n{:#?}", v);
        Ok(v)
    }

    /// Extracts the component id from the given PathBuf.
    ///
    /// Example:
    /// `testfiles/modules/footer/css` will result in `footer`.
    fn extract_component_ids_from_pathbuf(
        &self,
        p: &PathBuf,
    ) -> Result<String, LewpError> {
        let os_str = match p.parent() {
            Some(parent) => match parent.file_name() {
                Some(f) => f,
                None => {
                    return Err(LewpError {
                        kind: LewpErrorKind::FileHierarchy,
                        message: format!(
                            "Could not extract file name from parent of PathBuf: {:#?}",
                            p
                        ),
                        source_component: Arc::new(ComponentInformation::core(
                            "extract_component_ids_from_pathbuf",
                        )),
                    })
            }
            },
            None => {
                    return Err(LewpError {
                        kind: LewpErrorKind::FileHierarchy,
                        message: format!(
                            "Could not extract parent from PathBuf: {:#?}",
                            p
                        ),
                        source_component: Arc::new(ComponentInformation::core(
                            "extract_component_ids_from_pathbuf",
                        )),
                    })
            }
        };
        let id = match os_str.to_str() {
            Some(s) => s.to_string(),
            None => {
                return Err(LewpError {
                    kind: LewpErrorKind::FileHierarchy,
                    message: format!(
                        "Could not create String from OsStr: {:#?}",
                        os_str
                    ),
                    source_component: Arc::new(ComponentInformation::core(
                        "extract_component_ids_from_pathbuf",
                    )),
                })
            }
        };
        Ok(id)
    }

    /// Removes `../` from the given string to isolate the filepath to a base
    /// directory.
    fn isolate_path(&self, path: &str) -> String {
        let s = String::from(path);
        let mut s = s.split('/').collect::<Vec<&str>>();
        s.retain(|&e| !e.contains(".."));
        s.join("/")
    }

    /// Collects all folders in the given subfolder. Can be used to find eg.
    /// all modules available
    ///
    /// **For internal use only.**
    fn collect_foldernames(
        &self,
        subfolder: &PathBuf,
    ) -> Result<Vec<PathBuf>, LewpError> {
        let subfolder = self.mountpoint.join(subfolder);
        if !subfolder.is_dir() {
            return Err(LewpError {
                kind: LewpErrorKind::FileHierarchy,
                message: format!(
                    "Given input is not a folder: {}",
                    subfolder.display()
                ),
                source_component: Arc::new(ComponentInformation::core(
                    "collect_foldernames",
                )),
            });
        }
        let mut foldernames = vec![];
        for entry in walkdir::WalkDir::new(&subfolder) {
            let entry = match entry {
                Ok(v) => v.into_path(),
                Err(msg) => {
                    return Err(LewpError {
                        kind: LewpErrorKind::FileHierarchy,
                        message: msg.to_string(),
                        source_component: Arc::new(ComponentInformation::core(
                            "collect_foldernames",
                        )),
                    });
                }
            };
            if !entry.is_dir() {
                // skip files because we only want to get the folders in the list
                continue;
            }
            foldernames.push(entry);
        }
        Ok(foldernames)
    }
}

impl Default for FileHierarchy {
    /// Creates a new instance of the file hierarchy with the [FileHierarchy::new] function.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use {
        super::{
            Component,
            ComponentInformation,
            ComponentType,
            FileHierarchy,
            FileHierarchyBuilder,
            Level,
        },
        crate::LewpError,
        std::sync::Arc,
    };
    struct Css {
        id: String,
        fh: Arc<FileHierarchy>,
    }
    impl Component for Css {
        type Content = ();
        type ContentParameter = ();
        fn component_information(&self) -> Arc<ComponentInformation> {
            Arc::new(ComponentInformation {
                id: self.id.clone(),
                level: Level::Module,
                kind: ComponentType::Css,
            })
        }
        fn content(
            &self,
            _: Self::ContentParameter,
        ) -> Result<Self::Content, LewpError> {
            Ok(())
        }
        fn file_hierarchy(&self) -> Arc<FileHierarchy> {
            self.fh.clone()
        }
    }
    struct Js {
        fh: Arc<FileHierarchy>,
    }
    impl Component for Js {
        type Content = ();
        type ContentParameter = ();
        fn component_information(&self) -> Arc<ComponentInformation> {
            Arc::new(ComponentInformation {
                id: "hello-world".to_string(),
                level: Level::Page,
                kind: ComponentType::JavaScript,
            })
        }
        fn content(
            &self,
            _: Self::ContentParameter,
        ) -> Result<Self::Content, LewpError> {
            Ok(())
        }
        fn file_hierarchy(&self) -> Arc<FileHierarchy> {
            self.fh.clone()
        }
    }

    #[test]
    fn folder_name_generation() {
        let fh = Arc::new(FileHierarchy::new());
        let css = Css {
            id: String::from("module-id"),
            fh: fh.clone(),
        };
        let js = Js { fh: fh.clone() };
        assert_eq!(
            "./modules/module-id/css",
            fh.folder(&css).to_str().unwrap()
        );
        assert_eq!("./pages/hello-world/js", fh.folder(&js).to_str().unwrap());
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
        use std::path::PathBuf;
        let fh = Arc::new(
            FileHierarchyBuilder::new()
                .mountpoint(PathBuf::from("testfiles"))
                .build(),
        );
        let css = Css {
            id: String::from("hello-world"),
            fh: fh.clone(),
        };
        let mut filenames = match fh.get_file_list(&css) {
            Ok(f) => f,
            Err(e) => {
                panic!("{}", e)
            }
        };
        let mut reference = vec![
            PathBuf::from("modules/hello-world/css/primary.css"),
            PathBuf::from("modules/hello-world/css/secondary.css"),
        ];
        assert_eq!(filenames.sort(), reference.sort());
    }

    #[test]
    fn collect_foldernames() {
        use std::path::PathBuf;
        let fh = Arc::new(
            FileHierarchyBuilder::new()
                .mountpoint(PathBuf::from("testfiles"))
                .build(),
        );
        let css = Css {
            id: String::from("hello-world"),
            fh: fh.clone(),
        };
        let mut filenames =
            match fh.collect_foldernames(&PathBuf::from("modules")) {
                Ok(f) => f,
                Err(e) => {
                    panic!("{}", e)
                }
            };
        let mut reference = vec![PathBuf::from("modules/hello-world")];
        assert_eq!(filenames.sort(), reference.sort());
    }

    #[test]
    fn collect_component_ids() {
        use std::path::PathBuf;
        let fh = Arc::new(
            FileHierarchyBuilder::new()
                .mountpoint(PathBuf::from("testfiles"))
                .build(),
        );
        let mut component_ids =
            match fh.collect_component_ids(ComponentType::Css, Level::Module) {
                Ok(ids) => ids,
                Err(e) => {
                    panic!("{}", e)
                }
            };
        let mut reference = vec!["footer", "hello-world", "navigation"];
        assert_eq!(component_ids.sort(), reference.sort());
    }
}
