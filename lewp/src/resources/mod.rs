//! Contains access to resources in a given [archive](crate::archive).

use {
    crate::archive::{Archive, ArchiveComponent},
    std::path::PathBuf,
};

pub(crate) mod css;
mod image;
mod js;
mod resource_type;
mod web_interface;
//mod text;

pub use {
    css::{Css, CssOptions},
    image::Image,
    js::{Js, JsOptions},
    resource_type::ResourceType,
    //text::Text,
    web_interface::WebInterface,
};

/// Defines an id for a resource, for example `sample-image.jpg`.
pub type ResourceId = PathBuf;

/// The different storage level available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceLevel {
    /// The [Component](crate::component::Component) level.
    Component,
    /// The [Page](crate::page::Page) level.
    Page,
}

impl std::fmt::Display for ResourceLevel {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        use ResourceLevel::*;
        let s = match self {
            Component => "components",
            Page => "pages",
        };
        write!(f, "{s}")
    }
}

impl TryFrom<&std::ffi::OsStr> for ResourceLevel {
    type Error = anyhow::Error;
    fn try_from(value: &std::ffi::OsStr) -> Result<Self, Self::Error> {
        let r = match value.to_os_string().into_string() {
            Ok(r) => r,
            Err(e) => return Err(anyhow::anyhow!("{e:?}")),
        };
        match &r[..] {
            "components" => Ok(ResourceLevel::Component),
            "pages" => Ok(ResourceLevel::Page),
            _ => {
                Err(anyhow::anyhow!("Unknown resource level: {value:?} given!"))
            }
        }
    }
}

/// A specific resource with additional information required to be used in a webpage.
///
/// Especially used in combination with [ArchiveCache](crate::archive::ArchiveCache)
/// because the [WebInterface] is no longer available after loading from disk.
/// This is a side effect of the [ArchiveComponent::load] method that takes
/// the [Archive] as generic parameter.
#[derive(Debug)]
pub struct Resource<R: crate::archive::ArchiveComponent> {
    model: R,
    /// The web root path where this resource is available.
    pub web_root: PathBuf,
    /// The path to the resource.
    pub path: PathBuf,
}

impl<R: crate::archive::ArchiveComponent> Resource<R> {
    /// Loads the given resource from disk.
    pub fn load<A: Archive>(
        options: <R as ArchiveComponent>::Options,
    ) -> anyhow::Result<Self> {
        let model = R::load::<A>(options)?;
        let path = A::path(model.details());
        Ok(Self {
            model,
            web_root: A::web_root(),
            path,
        })
    }
}

impl<R: crate::archive::ArchiveComponent> std::ops::Deref for Resource<R> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        &self.model
    }
}
