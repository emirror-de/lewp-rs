use {
    super::{component_type::ComponentType, information::ComponentInformation},
    crate::fh::{FileHierarchy, Level},
    std::sync::Arc,
};

/// A lewp component. Anything inside the file hierarchy is a component (Files, Folders, Modules,
/// Pages etc.).
pub trait Component {
    /// Content type that the component type delivers.
    type Content;
    /// Parameter that can be used to decide about the content delivered.
    type ContentParameter;

    /// Returns the ComponentInformation instance. Required eg. for passing information to
    /// LewpError.
    fn component_information(&self) -> Arc<ComponentInformation>;
    /// Implementation of acquiring the content for this type of component.
    fn content<T: FileHierarchy>(
        &self,
        params: Self::ContentParameter,
    ) -> anyhow::Result<Self::Content>;

    /// The unique ID of the component.
    fn id(&self) -> String {
        self.component_information().id.clone()
    }
    /// Determines the level of the component on the file hierarchy.
    fn level(&self) -> Level {
        self.component_information().level
    }
    /// The component type.
    fn kind(&self) -> ComponentType {
        self.component_information().kind.clone()
    }
}
