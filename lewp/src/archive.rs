//! Provides an archive for your resources with the possibility to embed it in your binary.
#![doc = include_str!(concat!("../docs/archive.md"))]

use {
    crate::{
        component::{ComponentDetails, ComponentId},
        resources::{ResourceId, ResourceLevel, ResourceType, WebInterface},
    },
    anyhow::Context,
    rust_embed::RustEmbed,
    std::{
        borrow::Cow,
        path::{Path, PathBuf},
    },
};

mod cache;
mod component;
mod root;

pub use {cache::ArchiveCache, component::ArchiveComponent, root::ArchiveRoot};

/// Defines an archive at the given filesystem location. Uses [rust-embed](rust_embed)
/// under the hood to compile the archive files into the release binary.
///
/// For example:
/// ```rust
/// # use lewp::lewp_archive;
/// lewp_archive!(AssetsArchive, "testfiles");
/// ```
/// Will expand to:
/// ```rust
/// #[derive(::rust_embed::RustEmbed)]
/// #[folder = "testfiles"]
/// pub struct AssetsArchive;
/// ```
#[macro_export]
macro_rules! lewp_archive {
    ($name: ident, $folder: literal) => {
        /// User defined archive in the given folder.
        #[derive($crate::rust_embed::RustEmbed)]
        #[folder = $folder]
        pub struct $name;
        impl $crate::archive::ArchiveRoot for $name {
            fn root() -> ::std::path::PathBuf {
                ::std::path::PathBuf::from($folder)
            }
        }
    };
}

/// Definition of the archive on the file system.
pub trait Archive: ArchiveRoot + WebInterface
where
    Self: RustEmbed,
{
    /// Generates the path to the given [ComponentDetails] relative to [root](ArchiveRoot::root).
    fn path(details: &ComponentDetails) -> PathBuf;
    /// Collects all filenames in from the storage that are available for
    /// the given component.
    fn get_file_list(details: &ComponentDetails) -> Vec<PathBuf>;
    /// Gets a list of the component ids available for this [ResourceType] on the
    /// given [Level].
    fn collect_component_ids(
        kind: ResourceType,
        level: ResourceLevel,
    ) -> anyhow::Result<Vec<ComponentId>>;
    /// Extracts the different parts defining the link to the resource.
    ///
    /// The default implementation for a storage is:
    /// ```text
    /// resources/components/hello-world/css/my-crazy-resourcefile.ext
    /// ^         ^         ^           ^                    ^
    /// root      |         |           resource_type        |
    /// (unused)  level     component_id                 resource_id
    /// ```
    /// ## Examples
    fn parse(value: PathBuf) -> anyhow::Result<ComponentDetails> {
        // create default values
        let mut resource_id = None;

        let it = &mut value.iter().rev();
        // if the last entry has an extension, it points to a file. otherwise
        // it is specified to be a component wide resource.
        if value.extension().is_some() {
            resource_id = it.next().map(|r| ResourceId::from(r));
        }

        let resource_type = match it.next() {
            Some(r) => ResourceType::try_from(r)?,
            None => {
                return Err(anyhow::anyhow!(
                    "Could not extract resource_type from {value:?}"
                ));
            }
        };

        let component_id = match it.next() {
            Some(c) => {
                ComponentId::from(match c.to_os_string().into_string() {
                    Ok(r) => r,
                    Err(e) => return Err(anyhow::anyhow!("{e:?}")),
                })
            }
            None => {
                return Err(anyhow::anyhow!(
                    "Could not extract component_id from {value:?}"
                ));
            }
        };

        let level = match it.next() {
            Some(l) => ResourceLevel::try_from(l)?,
            None => {
                return Err(anyhow::anyhow!(
                    "Could not extract level from {value:?}"
                ));
            }
        };

        Ok(ComponentDetails {
            level,
            resource_type,
            component_id,
            resource_id,
        })
    }
}

impl<A: RustEmbed + ArchiveRoot + WebInterface> Archive for A {
    fn path(value: &ComponentDetails) -> PathBuf {
        let path = PathBuf::from(value.level.to_string())
            .join(value.component_id.to_string())
            .join(value.resource_type.to_string());
        let path = match &value.resource_id {
            Some(r) => path.join(r),
            None => path,
        };
        path
    }

    fn get_file_list(details: &ComponentDetails) -> Vec<PathBuf> {
        let subfolder = Path::new(&details.level.to_string())
            .join(Path::new(&details.component_id))
            .join(Path::new(&details.resource_type.to_string()));
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
        level: ResourceLevel,
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
                match Self::parse(PathBuf::from(e.clone().into_owned())) {
                    Ok(d) => Some(d.component_id),
                    Err(_) => None,
                }
            })
            .collect();
        component_ids.dedup();
        log::debug!("Found component ids:\n{:#?}", component_ids);

        Ok(component_ids)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            lewp_archive,
            resources::{Resource, ResourceLevel, ResourceType},
        },
        mime::Mime,
        std::sync::Arc,
    };
    const RENDER_CRITICAL: &str = "#nav {display:none}";
    const NON_RENDER_CRITICAL: &str = "#nav {color: #000}";
    const JS_SCRIPT_COMBINED: &str = "console.log('hello-world');";

    #[derive(Debug)]
    struct Css {
        details: ComponentDetails,
        pub render_critical: Arc<String>,
        pub non_render_critical: Arc<String>,
    }

    impl ArchiveComponent for Css {
        type Options = ();
        fn load<A: Archive>(_options: Self::Options) -> anyhow::Result<Self>
        where
            Self: Sized,
        {
            Ok(Css {
                details: ComponentDetails {
                    level: ResourceLevel::Component,
                    resource_type: ResourceType::Css,
                    resource_id: None,
                    component_id: "hello-world".into(),
                },
                render_critical: Arc::new(RENDER_CRITICAL.into()),
                non_render_critical: Arc::new(NON_RENDER_CRITICAL.into()),
            })
        }
        fn mime_type() -> Mime {
            mime::TEXT_CSS
        }
        fn details(&self) -> &ComponentDetails {
            &self.details
        }
    }

    #[derive(Debug)]
    struct Js {
        details: ComponentDetails,
        pub some_combined_script: Arc<String>,
    }

    impl ArchiveComponent for Js {
        type Options = ();
        fn load<A: Archive>(_options: Self::Options) -> anyhow::Result<Self>
        where
            Self: Sized,
        {
            Ok(Js {
                details: ComponentDetails {
                    level: ResourceLevel::Component,
                    resource_type: ResourceType::JavaScript,
                    resource_id: None,
                    component_id: "hello-world".into(),
                },
                some_combined_script: Arc::new(JS_SCRIPT_COMBINED.into()),
            })
        }
        fn mime_type() -> Mime {
            mime::APPLICATION_JAVASCRIPT
        }
        fn details(&self) -> &ComponentDetails {
            &self.details
        }
    }

    lewp_archive!(ResourceArchive, "testfiles");
    impl WebInterface for ResourceArchive {}

    fn archive_cache_example() -> ArchiveCache {
        ArchiveCache::default()
    }

    #[test]
    fn archive_example() {
        let mut cache = archive_cache_example();

        let css =
            Arc::new(Resource::<Css>::load::<ResourceArchive>(()).unwrap());
        let css_component_details = ComponentDetails {
            level: ResourceLevel::Component,
            resource_type: ResourceType::Css,
            resource_id: None,
            component_id: "hello-world".into(),
        };
        cache.insert(css);

        let js = Arc::new(Resource::<Js>::load::<ResourceArchive>(()).unwrap());
        let js_component_details = ComponentDetails {
            level: ResourceLevel::Component,
            resource_type: ResourceType::JavaScript,
            resource_id: None,
            component_id: "hello-world".into(),
        };
        cache.insert(js);

        let css_queried = cache.query::<Css>(&css_component_details).unwrap();
        assert_eq!(*css_queried.render_critical, RENDER_CRITICAL);
        assert_eq!(*css_queried.non_render_critical, NON_RENDER_CRITICAL);

        let js_queried = cache.query::<Js>(&js_component_details).unwrap();
        assert_eq!(*js_queried.some_combined_script, JS_SCRIPT_COMBINED);
    }
}
