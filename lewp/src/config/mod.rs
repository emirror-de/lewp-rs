//! Configuration data structures used by [lewp](crate).

use {
    crate::{css::Register as CssRegister, fh::FileHierarchy, Lewp},
    std::sync::Arc,
};

/// Various config values for adjusting the modules behavior.
pub struct ModuleConfig {}

impl ModuleConfig {
    /// Creates a new instance with default values.
    pub fn new() -> Self {
        Self {}
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
    /// The file hierarchy where the page is working on.
    pub fh: Option<Arc<FileHierarchy>>,
    /// The CSS register containing all required CSS data of modules and pages.
    pub css_register: Option<Arc<CssRegister>>,
}

impl PageConfig {
    /// Creates a new instance with default values.
    pub fn new(
        viewport_tag: bool,
        fh: Option<Arc<FileHierarchy>>,
        css_register: Option<Arc<CssRegister>>,
    ) -> Self {
        Self {
            viewport_tag,
            fh,
            css_register,
        }
    }
}

impl From<&Lewp> for PageConfig {
    fn from(lewp: &Lewp) -> Self {
        let fh = match &lewp.fh {
            Some(f) => Some(f.clone()),
            None => None,
        };
        let css_register = match &lewp.css_register {
            Some(c) => Some(c.clone()),
            None => None,
        };
        Self {
            viewport_tag: true,
            fh,
            css_register,
        }
    }
}

impl Default for PageConfig {
    fn default() -> Self {
        Self::new(true, None, None)
    }
}
