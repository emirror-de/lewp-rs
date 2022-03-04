//! Configuration data structures used by [lewp](crate).

/// Various config values for adjusting the modules behavior.
pub struct ModuleConfig {
    /// If true, the module gets rendered without a wrapping `<div>` tag.
    pub skip_wrapper: bool,
}

impl ModuleConfig {
    /// Creates a new instance with default values.
    pub fn new() -> Self {
        Self {
            skip_wrapper: false,
        }
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
}

impl PageConfig {
    /// Creates a new instance with default values.
    pub fn new() -> Self {
        Self { viewport_tag: true }
    }
}

impl Default for PageConfig {
    fn default() -> Self {
        Self::new()
    }
}
