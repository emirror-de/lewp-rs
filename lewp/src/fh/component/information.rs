use {super::ComponentType, crate::fh::Level};

/// Defines the information of the lewp component.
///
/// The actual component should contain an instance of this struct.
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct ComponentInformation {
    /// The unique ID of the component.
    pub id: String,
    /// Determines the level of the component on the file hierarchy.
    pub level: Level,
    /// The component type.
    pub kind: ComponentType,
}

impl ComponentInformation {
    /// Creates the component information for a core event.
    pub fn core(component_name: &str) -> Self {
        Self {
            id: "".to_string(),
            level: Level::Core,
            kind: ComponentType::Core(component_name.to_string()),
        }
    }
}
