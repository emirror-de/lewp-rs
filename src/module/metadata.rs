use crate::config::ModuleConfig;

/// Contains metadata of the module.
pub trait Metadata {
    /// Returns the unique module id.
    fn id(&self) -> &str;

    /// The configuration of the module.
    fn config(&self) -> &ModuleConfig;
}
