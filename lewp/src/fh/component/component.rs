use {
    super::{component_type::ComponentType, information::ComponentInformation},
    crate::{
        fh::{FileHierarchy, Level},
        LewpError,
    },
    std::{path::PathBuf, sync::Arc},
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
    /// Returns a reference to the file hierarchy instance attached to this component.
    fn file_hierarchy(&self) -> Arc<FileHierarchy>;
    /// Implementation of acquiring the content for this type of component.
    fn content(
        &self,
        params: Self::ContentParameter,
    ) -> Result<Self::Content, LewpError>;

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
    /// Returns the folder name of the component according to the file hierarchy.
    fn folder_name(&self) -> PathBuf
    where
        Self: Sized,
    {
        self.file_hierarchy().folder(self)
    }
}
