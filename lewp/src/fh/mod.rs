//! Defines the file hierarchy of [lewp](crate).

use {
    crate::{LewpError, LewpErrorKind},
    rust_embed::RustEmbed,
    std::{
        borrow::Cow,
        path::{Path, PathBuf},
        sync::Arc,
    },
};

//mod builder;
mod component;
mod level;

pub use {
    //builder::FileHierarchyBuilder,
    component::{Component, ComponentInformation, ComponentType, ResourceType},
    level::Level,
};

/// Defines the behavior of the file hierarchy.
pub trait FileHierarchy
where
    Self: RustEmbed,
{
    /// Generates the folder path according to the file hierarchy. The folder
    /// that contains the `file_type` always corresponds to the extension of the
    /// files contained.
    fn folder<COMP: Component>(component: &COMP) -> PathBuf;
    /// Collects all filenames in the given component. The resulting
    /// vector contains the filepath including the mountpoint of the FileHierarchy.
    /// This function is not recursive.
    fn get_file_list<COMP: Component>(component: &COMP) -> Vec<PathBuf>;
    /// Gets a list of the component ids available for this [ComponentType] on the
    /// given [Level].
    fn collect_component_ids(
        kind: ComponentType,
        level: Level,
    ) -> anyhow::Result<Vec<String>>;
    /// Extracts the component id from the given PathBuf.
    ///
    /// Example:
    /// `testfiles/components/footer/css` will result in `footer`.
    fn extract_component_ids_from_pathbuf(
        p: &PathBuf,
    ) -> anyhow::Result<String> {
        if p.parent().is_none() || p.parent().unwrap().parent().is_none() {
            return Err(anyhow::anyhow!("Invalid file path: {p:?}"));
        }
        let os_str = match p.parent().unwrap().parent() {
            Some(parent) => match parent.file_name() {
                Some(f) => f,
                None => {
                    return Err(anyhow::anyhow!("{}", LewpError {
                        kind: LewpErrorKind::FileHierarchy,
                        message: format!(
                            "Could not extract file name from parent of PathBuf: {p:#?}"
                        ),
                        source_component: Arc::new(ComponentInformation::core(
                            "extract_component_ids_from_pathbuf",
                        )),
                    }))
            }
            },
            None => {
                    return Err(anyhow::anyhow!("{}", LewpError {
                        kind: LewpErrorKind::FileHierarchy,
                        message: format!(
                            "Could not extract parent from PathBuf: {p:#?}"
                        ),
                        source_component: Arc::new(ComponentInformation::core(
                            "extract_component_ids_from_pathbuf",
                        )),
                    }))
            }
        };
        let id = match os_str.to_str() {
            Some(s) => s.to_string(),
            None => {
                return Err(anyhow::anyhow!(
                    "{}",
                    LewpError {
                        kind: LewpErrorKind::FileHierarchy,
                        message: format!(
                            "Could not create String from OsStr: {os_str:#?}"
                        ),
                        source_component: Arc::new(ComponentInformation::core(
                            "extract_component_ids_from_pathbuf",
                        )),
                    }
                ))
            }
        };
        Ok(id)
    }
}

/// Mounts the file hierarchy at the given folder.
#[macro_export]
macro_rules! file_hierarchy {
    ($name: ident, $folder: literal) => {
        /// Storage definition of the file hierarchy.
        #[derive(::rust_embed::RustEmbed)]
        #[folder = $folder]
        pub struct $name;
    };
}

impl<T: RustEmbed> FileHierarchy for T {
    fn folder<COMP: Component>(component: &COMP) -> PathBuf {
        let mut path = PathBuf::from(component.level().to_string());
        path.push(&component.id());
        path.push(component.kind().to_string());
        path
    }

    fn get_file_list<COMP: Component>(component: &COMP) -> Vec<PathBuf> {
        let subfolder = Path::new(&component.level().to_string())
            .join(Path::new(&component.id()))
            .join(Path::new(&component.kind().to_string()));
        log::debug!("Generated subfolder: {}", subfolder.display());

        let mut filenames: Vec<PathBuf> = <Self as RustEmbed>::iter()
            .filter(|f| {
                Path::new(&f.clone().into_owned()).starts_with(&subfolder)
            })
            .map(|f| PathBuf::from(f.into_owned()))
            .collect();

        filenames.sort();
        filenames
    }

    fn collect_component_ids(
        kind: ComponentType,
        level: Level,
    ) -> anyhow::Result<Vec<String>> {
        let pattern_path = Path::new(&level.to_string())
            .join("*")
            .join(kind.to_string());

        let pattern_path = match kind.extension() {
            Some(e) => pattern_path.join("*".to_string() + &e),
            None => pattern_path,
        };

        let pattern_path = match pattern_path.to_str() {
            Some(p) => p,
            None => {
                return Err(anyhow::anyhow!(
                    "{}",
                    LewpError {
                        kind: LewpErrorKind::FileHierarchy,
                        message: String::from(
                            "Error converting filepath to string!",
                        ),
                        source_component: Arc::new(ComponentInformation::core(
                            "get_component_ids",
                        )),
                    }
                ))
            }
        };
        let pattern = glob::Pattern::new(&pattern_path)?;
        log::debug!("Created pattern to search for: {pattern}");

        let filenames: Vec<Cow<'static, str>> = <Self as RustEmbed>::iter()
            .filter(|f| pattern.matches(&f.clone().into_owned()))
            .collect();
        log::debug!("Found filenames that match:\n{filenames:?}");

        let mut component_ids: Vec<String> = filenames
            .iter()
            .filter_map(|e| {
                Self::extract_component_ids_from_pathbuf(&PathBuf::from(
                    e.clone().into_owned(),
                ))
                .ok()
            })
            .collect();
        component_ids.dedup();
        log::debug!("Found component ids:\n{:#?}", component_ids);

        Ok(component_ids)
    }
}

#[cfg(test)]
mod tests {
    file_hierarchy!(TestHierarchy, "testfiles");

    use {
        super::{
            Component,
            ComponentInformation,
            ComponentType,
            FileHierarchy,
            Level,
        },
        std::sync::Arc,
    };

    struct Css {
        id: String,
    }
    impl Component for Css {
        type Content = ();
        type ContentParameter = ();
        fn component_information(&self) -> Arc<ComponentInformation> {
            Arc::new(ComponentInformation {
                id: self.id.clone(),
                level: Level::Component,
                kind: ComponentType::Css,
            })
        }
        fn content<T: FileHierarchy>(
            &self,
            _: Self::ContentParameter,
        ) -> anyhow::Result<Self::Content> {
            Ok(())
        }
    }
    struct Js {}
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
        fn content<T: FileHierarchy>(
            &self,
            _: Self::ContentParameter,
        ) -> anyhow::Result<Self::Content> {
            Ok(())
        }
    }

    #[test]
    fn folder_name_generation() {
        let css = Css {
            id: String::from("module-id"),
        };
        let js = Js {};
        assert_eq!(
            "components/module-id/css",
            TestHierarchy::folder(&css).to_str().unwrap()
        );
        assert_eq!(
            "pages/hello-world/js",
            TestHierarchy::folder(&js).to_str().unwrap()
        );
    }

    #[test]
    fn collect_filenames() {
        use std::path::PathBuf;

        let css = Css {
            id: String::from("hello-world"),
        };
        let mut filenames = TestHierarchy::get_file_list(&css);
        let mut reference = vec![
            PathBuf::from("components/hello-world/css/primary.css"),
            PathBuf::from("components/hello-world/css/secondary.css"),
        ];
        assert_eq!(filenames.sort(), reference.sort());
    }

    #[test]
    fn collect_component_ids() {
        use super::{ComponentType, FileHierarchy, Level};

        let mut component_ids = match TestHierarchy::collect_component_ids(
            ComponentType::Css,
            Level::Component,
        ) {
            Ok(ids) => ids,
            Err(e) => {
                panic!("{}", e)
            }
        };
        let mut reference = vec!["footer", "hello-world", "navigation"];
        component_ids.sort();
        reference.sort();
        assert_eq!(component_ids, reference);
    }
}
