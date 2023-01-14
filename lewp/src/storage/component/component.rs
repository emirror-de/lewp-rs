use {
    super::super::{Level, ResourceType, Route, Storage},
    crate::component::ComponentId,
    std::path::{Path, PathBuf},
};

/// Any resource (see [ResourceType]) inside the storage is a component (Images, CSS, JS etc.).
pub trait StorageComponent {
    /// Content type that the component type delivers.
    type Content;
    /// Parameter that can be used to decide about the content delivered.
    type ContentParameter;

    /// Implementation of acquiring the content for this type of component.
    fn content<T: Storage>(
        &self,
        params: Self::ContentParameter,
    ) -> anyhow::Result<Self::Content>;

    /// The unique ID of the component.
    fn id(&self) -> ComponentId;
    /// Determines the level of the component on the file hierarchy.
    fn level(&self) -> Level;
    /// The resource type.
    fn kind(&self) -> ResourceType;
    /// Returns the router path to the given component file.
    fn route<FH: Storage + Route>(&self, filename: &str) -> PathBuf {
        Path::new(FH::route())
            .join(self.level().to_string())
            .join(self.id().to_string())
            .join(self.kind().to_string())
            .join(filename)
    }
}
