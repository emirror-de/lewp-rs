use {
    super::{component_type::ComponentType, information::ComponentInformation},
    crate::fh::{FileHierarchy, Level, Route},
    std::{
        path::{Path, PathBuf},
        sync::Arc,
    },
};

/// A lewp component. Any resource inside the file hierarchy is a component (Images, CSS, JS etc.).
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
    /// Returns the router path to the given component file.
    fn route<FH: FileHierarchy + Route>(&self, filename: &str) -> PathBuf {
        let c = self.component_information();
        Path::new(FH::route())
            .join(c.level.to_string())
            .join(c.id.to_string())
            .join(c.kind.to_string())
            .join(filename)
    }
}
