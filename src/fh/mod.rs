//! Defines the file hierarchy of [lewp](crate).

use {
    crate::{LewpError, LewpErrorKind},
    std::{
        path::{Path, PathBuf},
        rc::Rc,
    },
};

mod builder;
mod component;
mod level;

pub use {
    builder::FileHierarchyBuilder,
    component::{Component, ComponentInformation, ComponentType},
    level::Level,
};

/// File hierarchy definition, handles file path generation.
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
    pub fn folder<COMP: Component>(&self, component: &COMP) -> PathBuf {
        let mut path = self.base_directory.clone();
        path.push(component.level().to_string());
        path.push(component.id().to_string());
        path.push(component.kind().to_string());
        path
    }

    /// Collects all filenames recursively in the given component. The resulting
    /// vector is referenced to the base directory given in the FileHierarchy instance.
    pub fn get_file_list<COMP: Component>(
        &self,
        component: &COMP,
    ) -> Result<Vec<PathBuf>, LewpError> {
        let subfolder = self.base_directory.join(Path::new(&format!(
            "{}/{}/{}",
            component.level(),
            component.id(),
            component.kind()
        )));
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
            //let entry = match self.remove_base_dir(&subfolder, &entry) {
            let entry = match self.remove_base_dir(&self.base_directory, &entry)
            {
                Ok(p) => p,
                Err(msg) => {
                    log::error!("{}", msg);
                    continue;
                }
            };
            filenames.push(self.base_directory.join(entry));
        }
        Ok(filenames)
    }

    fn remove_base_dir(
        &self,
        base_dir: &Path,
        input_path: &Path,
    ) -> Result<PathBuf, String> {
        match pathdiff::diff_paths(input_path, base_dir) {
            Some(p) => Ok(p),
            None => match input_path.to_str() {
                Some(v) => Err(format!("Could not remove base dir of {}", v)),
                None => Err("Could not remove base dir!".to_string()),
            },
        }
    }

    /// Gets a list of the component ids available for this [ComponentType] on the
    /// given [Level].
    pub fn get_component_ids(
        &self,
        kind: ComponentType,
        level: Level,
    ) -> Vec<String> {
        vec![]
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
        std::rc::Rc,
    };
    struct Css {
        id: String,
        fh: Rc<FileHierarchy>,
    }
    impl Component for Css {
        type Content = ();
        fn component_information(&self) -> Rc<ComponentInformation> {
            Rc::new(ComponentInformation {
                id: self.id.clone(),
                level: Level::Module,
                kind: ComponentType::Css,
            })
        }
        fn content(&self) -> Result<Self::Content, LewpError> {
            Ok(())
        }
        fn file_hierarchy(&self) -> Rc<FileHierarchy> {
            self.fh.clone()
        }
    }
    struct Js {
        fh: Rc<FileHierarchy>,
    }
    impl Component for Js {
        type Content = ();
        fn component_information(&self) -> Rc<ComponentInformation> {
            Rc::new(ComponentInformation {
                id: "hello-world".to_string(),
                level: Level::Page,
                kind: ComponentType::JavaScript,
            })
        }
        fn content(&self) -> Result<Self::Content, LewpError> {
            Ok(())
        }
        fn file_hierarchy(&self) -> Rc<FileHierarchy> {
            self.fh.clone()
        }
    }

    #[test]
    fn folder_name_generation() {
        let fh = Rc::new(FileHierarchy::new());
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
        let fh = Rc::new(
            FileHierarchyBuilder::new()
                .base_directory(PathBuf::from("testfiles"))
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
}
