use {
    super::{Level, LewpError},
    std::{fmt::Debug, rc::Rc},
};

/// Defines the information of the lewp component.
///
/// The actual component should contain an instance of this struct.
pub struct ComponentInformation {
    /// The unique ID of the component.
    pub id: String,
    /// Determines the level of the component on the file hierarchy.
    pub level: Level,
    /// The component type.
    pub kind: String,
}

/// A lewp component. Anything inside the file hierarchy is a component (Files, Folders, Modules,
/// Pages etc.).
pub trait Component {
    /// Content type that the component type delivers.
    type Content;

    /// Returns the ComponentInformation instance. Required eg. for passing information to
    /// LewpError.
    fn component_information(&self) -> Rc<ComponentInformation>;
    /// Implementation of acquiring the content for this type of component.
    fn content(&self) -> Result<Self::Content, LewpError>;

    /// The unique ID of the component.
    fn id(&self) -> String {
        self.component_information().id.clone()
    }
    /// Determines the level of the component on the file hierarchy.
    fn level(&self) -> Level {
        self.component_information().level
    }
    /// The component type.
    fn kind(&self) -> String {
        self.component_information().kind.clone()
    }
    /// The folder name where the components of this type are stored.
    fn folder_name(&self) -> String {
        self.kind()
    }
}
//#[derive(Debug, Clone)]
//pub struct Component {
//    /// The unique ID of the component.
//    pub id: String,
//    /// Determines the level of the component on the file hierarchy.
//    pub level: Level,
//}
//
//impl Component {
//    /// Creates a new component instance.
//    pub fn new(id: &str, level: Level) -> Self {
//        Self {
//            id: id.to_string(),
//            level,
//        }
//    }
//}
