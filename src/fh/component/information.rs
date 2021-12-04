use {super::ComponentType, crate::fh::Level};

/// Defines the information of the lewp component.
///
/// The actual component should contain an instance of this struct.
pub struct ComponentInformation {
    /// The unique ID of the component.
    pub id: String,
    /// Determines the level of the component on the file hierarchy.
    pub level: Level,
    /// The component type.
    pub kind: ComponentType,
}
