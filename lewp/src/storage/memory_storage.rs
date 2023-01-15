use {
    crate::{
        component::ComponentId,
        storage::{Level, StorageComponent, StorageRegister},
    },
    std::collections::HashMap,
};

mod css;

pub use css::CssQueryOptions;

/// Loads resources from disk and stores them in memory as long
/// as your application is running.
pub struct MemoryStorage<SC: StorageComponent>
where
    Self: StorageRegister,
{
    register: HashMap<(ComponentId, Level), SC::Content>,
}
