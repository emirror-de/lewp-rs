//! CSS modification functions especially required by lewp.

mod component;

use crate::fh::{Component as FHComponent, FileHierarchy};

pub use component::{Component, ComponentBuilder};

/// Defines the level of completeness.
pub enum CssCompleteness {
    /// The entire CSS.
    Full,
    /// Only render critical parts, at least everything that affects
    /// [cumulative layout shift](https://web.dev/cls/).
    RenderCritical,
    /// Only non-render critical parts.
    NonRenderCritical,
}

/// Options for CSS processing.
pub struct CssOptions {}

/// Manages the CSS of lewp components in the given file hierarchy.
pub struct Css {
    fh: FileHierarchy,
    options: CssOptions,
}

impl Css {
    /// Creates a new Css instance.
    pub fn new(fh: FileHierarchy, options: CssOptions) -> Self {
        Self { fh, options }
    }

    /// Queries the CSS of the given component using the given options.
    pub fn query(
        component: &FHComponent,
        entity: CssCompleteness,
    ) -> Result<(), ()> {
        Ok(())
    }
}
