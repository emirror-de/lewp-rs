mod component_type;
use super::Level;
pub use component_type::ComponentType;

/// A lewp component. Anything inside the file hierarchy is a component (Files, Folders, Modules,
/// Pages etc.).
#[derive(Debug, Clone)]
pub struct Component {
    /// The unique ID of the component.
    pub id: String,
    /// Determines the level of the component on the file hierarchy.
    pub level: Level,
    /// Determines the type of the component.
    pub kind: ComponentType,
}

impl Component {
    /// Creates a new component instance.
    pub fn new(id: &str, level: Level, kind: ComponentType) -> Self {
        Self {
            id: id.to_string(),
            level,
            kind,
        }
    }
}
