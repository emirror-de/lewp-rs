use {
    super::{Level, Storage},
    crate::component::ComponentId,
    std::sync::Arc,
};

/// Collection of multiple [StorageComponent](super::StorageComponent)s. Resources can be queried to
/// use them in your [Page](crate::page::Page) or [Component](crate::component::Component).
pub trait StorageRegister
where
    Self: Sized,
{
    /// The options for the register.
    type Options;
    /// Options that can be passed to a query.
    type QueryOptions;
    /// The content returned from the register. This content might be a different
    /// type than defined in [StorageComponent](super::StorageComponent), especially when implementing a
    /// resource with a single source but multiple output. See [`MemoryStorage<Css>`](super::MemoryStorage).
    type Content;
    /// Creates a new instance of the register. In here loading and initial
    /// processing of resources should be done.
    fn initialize<S: Storage>(options: Self::Options) -> anyhow::Result<Self>;
    /// Queries a resource from the register. Returns `None` if the resource
    /// could not be found.
    fn query(
        &self,
        id: ComponentId,
        level: Level,
        options: Self::QueryOptions,
    ) -> Option<Arc<Self::Content>>;
    /// Borrow the register [options](Self::Options).
    fn options(&self) -> &Self::Options;
}
