//! Different storage possibilities for your resources.

use {
    anyhow::Context,
    rust_embed::RustEmbed,
    std::{
        borrow::Cow,
        path::{Path, PathBuf},
    },
};

mod component;
mod level;
mod memory_storage;
mod register;
mod web_interface;

pub use {
    component::{ResourceType, StorageComponent},
    level::Level,
    memory_storage::{CssQueryOptions, JsQueryOptions, MemoryStorage},
    register::StorageRegister,
    web_interface::WebInterface,
};

/// A storage definition. This is an abstraction of a file hierarchy specific
///to your project.
pub trait Storage
where
    Self: RustEmbed,
{
    /// Generates the folder path as the storage defines it.
    fn folder_path<COMP: StorageComponent>(component: &COMP) -> PathBuf;
    /// Collects all filenames in from the storage that are available for
    /// the given component.
    fn get_file_list<COMP: StorageComponent>(component: &COMP) -> Vec<PathBuf>;
    /// Gets a list of the component ids available for this [ResourceType] on the
    /// given [Level].
    fn collect_component_ids(
        kind: ResourceType,
        level: Level,
    ) -> anyhow::Result<Vec<String>>;
    /// Extracts the component id from the given PathBuf.
    ///
    /// Example:
    /// `testfiles/components/footer/css` will result in `footer`.
    fn extract_component_ids(p: &PathBuf) -> anyhow::Result<String> {
        if p.parent().is_none() || p.parent().unwrap().parent().is_none() {
            return Err(anyhow::anyhow!("Invalid file path: {p:?}"));
        }
        let os_str = match p.parent().unwrap().parent() {
            Some(parent) => match parent.file_name() {
                Some(f) => f,
                None => {
                    return Err(
                        anyhow::anyhow!(
                            "{}",
                            format!("Could not extract file name from parent of PathBuf: {p:#?}")
                        )
                    ).context("extract_component_ids_from_pathbuf".to_string());
                }
            },
            None => {
                return Err(anyhow::anyhow!(
                    "{}",
                    format!("Could not extract parent from PathBuf: {p:#?}")
                ))
                .context("extract_component_ids_from_pathbuf".to_string());
            }
        };
        let id = match os_str.to_str() {
            Some(s) => s.to_string(),
            None => {
                return Err(anyhow::anyhow!(
                    "{}",
                    format!("Could not create String from OsStr: {os_str:#?}")
                ))
                .context("extract_component_ids_from_pathbuf".to_string());
            }
        };
        Ok(id)
    }
}

/// Defines a storage at the given filesystem location.
///
/// For example:
/// ```rust
/// # use lewp::lewp_storage;
/// lewp_storage!(AssetsStorage, "testfiles");
/// ```
/// Will expand to:
/// ```rust
/// #[derive(::rust_embed::RustEmbed)]
/// #[folder = "testfiles"]
/// pub struct AssetsStorage;
/// ```
#[macro_export]
macro_rules! lewp_storage {
    ($name: ident, $folder: literal) => {
        /// Storage definition of the file hierarchy.
        #[derive(::rust_embed::RustEmbed)]
        #[folder = $folder]
        pub struct $name;
    };
}

impl<T: RustEmbed> Storage for T {
    fn folder_path<COMP: StorageComponent>(component: &COMP) -> PathBuf {
        let mut path = PathBuf::from(component.level().to_string());
        path.push(&component.id());
        path.push(component.kind().to_string());
        path
    }

    fn get_file_list<COMP: StorageComponent>(component: &COMP) -> Vec<PathBuf> {
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
        kind: ResourceType,
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
                    format!("Error converting filepath to string!")
                ))
                .context("get_component_ids".to_string());
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
                Self::extract_component_ids(&PathBuf::from(
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
    lewp_storage!(TestStorage, "testfiles");

    use super::{Level, ResourceType, Storage, StorageComponent};

    struct Css {
        id: String,
    }
    impl StorageComponent for Css {
        type Content = ();
        type ContentParameter = ();
        fn content<T: Storage>(
            &self,
            _: Self::ContentParameter,
        ) -> anyhow::Result<Self::Content> {
            Ok(())
        }
        fn id(&self) -> crate::component::ComponentId {
            self.id.clone()
        }
        fn level(&self) -> Level {
            Level::Component
        }
        fn kind(&self) -> ResourceType {
            ResourceType::Css
        }
    }
    struct Js {}
    impl StorageComponent for Js {
        type Content = ();
        type ContentParameter = ();
        fn content<T: Storage>(
            &self,
            _: Self::ContentParameter,
        ) -> anyhow::Result<Self::Content> {
            Ok(())
        }
        fn id(&self) -> crate::component::ComponentId {
            "hello-world".into()
        }
        fn level(&self) -> Level {
            Level::Page
        }
        fn kind(&self) -> ResourceType {
            ResourceType::JavaScript
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
            TestStorage::folder_path(&css).to_str().unwrap()
        );
        assert_eq!(
            "pages/hello-world/js",
            TestStorage::folder_path(&js).to_str().unwrap()
        );
    }

    #[test]
    fn collect_filenames() {
        use std::path::PathBuf;

        let css = Css {
            id: String::from("hello-world"),
        };
        let mut filenames = TestStorage::get_file_list(&css);
        let mut reference = vec![
            PathBuf::from("components/hello-world/css/primary.css"),
            PathBuf::from("components/hello-world/css/secondary.css"),
        ];
        assert_eq!(filenames.sort(), reference.sort());
    }

    #[test]
    fn collect_component_ids() {
        use super::{Level, ResourceType, Storage};

        let mut component_ids = match TestStorage::collect_component_ids(
            ResourceType::Css,
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
