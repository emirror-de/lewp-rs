//! Contains the global configuration for [lewp](crate).

use {
    crate::{
        css::{Register as CssRegister, RegisterOptions as CssRegisterOptions},
        fh::FileHierarchy,
    },
    std::sync::Arc,
};

/// The global configuration struct.
pub struct Lewp {
    /// The file hierarchy that can be shared throughout the entire website.
    pub fh: Option<Arc<FileHierarchy>>,
    /// The CSS register containing all required CSS data of modules and pages.
    pub css_register: Option<Arc<CssRegister>>,
}

impl Lewp {
    /// Creates a new [Lewp] instance.
    pub fn new() -> Self {
        Self {
            fh: None,
            css_register: None,
        }
    }

    /// Attaches the given [FileHierarchy] to the [Lewp] instance.
    pub fn with_file_hierarchy(mut self, fh: FileHierarchy) -> Self {
        self.fh = Some(Arc::new(fh));
        self
    }

    /// Creates a new [CssRegister] instance in the [Lewp] instance with the given
    /// [CSSRegisterOptions].
    ///
    /// **This method does nothing if no [FileHierarchy] has been attached earlier.**
    pub fn with_css_register(
        mut self,
        css_register_options: CssRegisterOptions,
    ) -> Self {
        self.css_register = match &self.fh {
            None => None,
            Some(f) => Some(Arc::new(
                CssRegister::new(f.clone(), css_register_options).unwrap(),
            )),
        };
        self
    }
}
