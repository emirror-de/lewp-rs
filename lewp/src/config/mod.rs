//! Configuration data structures used by [lewp](crate).

use {crate::fh::FileHierarchy, std::sync::Arc};

/// Various config values for adjusting the modules behavior.
pub struct ModuleConfig {
    /// If true, the module gets wrapped by a `<div>` tag.
    ///
    /// **Default value:** false.
    pub wrapper: bool,
}

impl ModuleConfig {
    /// Creates a new instance with default values.
    pub fn new() -> Self {
        Self { wrapper: false }
    }
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// The page configuration.
pub struct PageConfig {
    /// Creates a default viewport tag and appends it on rendering.
    pub viewport_tag: bool,
    /// The file hierarchy where the module is working on.
    pub fh: Option<Arc<FileHierarchy>>,
}

impl PageConfig {
    /// Creates a new instance with default values.
    pub fn new(fh: Option<Arc<FileHierarchy>>) -> Self {
        Self {
            viewport_tag: true,
            fh,
        }
    }
}

impl Default for PageConfig {
    fn default() -> Self {
        Self::new(None)
    }
}
