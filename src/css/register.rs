use {
    super::{Component, Entireness},
    crate::fh::{Component as FHComponent, FileHierarchy},
    std::sync::Arc,
};

/// Options for the Register.
pub struct RegisterOptions {}

impl RegisterOptions {
    /// Creates a new RegisterOptions instance.
    pub fn new() -> Self {
        Self {}
    }
}

/// Manages the CSS of lewp components in the given file hierarchy.
///
/// This register can be used in multi threaded environments as a shared
/// variable. It loads all components available in the given file hierarchy and
/// keeps them in memory, as long as this instance is available.
pub struct Register {
    fh: FileHierarchy,
    options: RegisterOptions,
    components: Arc<Vec<Component>>,
}

impl Register {
    /// Creates a new Register instance.
    pub fn new(fh: FileHierarchy, options: RegisterOptions) -> Self {
        Self {
            fh,
            options,
            components: Arc::new(vec![]),
        }
    }

    /// Queries the CSS of the given component using the given options.
    pub fn query(
        component: &FHComponent,
        entity: Entireness,
    ) -> Result<(), ()> {
        Ok(())
    }
}

impl Default for Register {
    fn default() -> Self {
        Self::new(FileHierarchy::new(), RegisterOptions::new())
    }
}
