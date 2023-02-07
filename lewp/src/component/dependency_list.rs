//! Implements the dependency list of a component. A dependency is a
//! component that is used within or by another component. The items in the list
//! are unique.

use {super::ComponentId, std::fmt::Display};

/// A list of components that are required by your
/// [ComponentModel](super::ComponentModel) implementation.
#[derive(Clone, Default)]
pub struct DependencyList {
    dependency_list: Vec<ComponentId>,
}

impl DependencyList {
    /// Adds the given [ComponentId] to the dependency list.
    pub fn push(&mut self, id: ComponentId) {
        if self.dependency_list.contains(&id) {
            return;
        }
        self.dependency_list.push(id);
    }

    /// Returns the list as [`Vec<ComponentId>`].
    pub fn list(&self) -> &Vec<ComponentId> {
        &self.dependency_list
    }

    /// True if the component list already contains the given [ComponentId].
    pub fn contains(&self, id: ComponentId) -> bool {
        log::debug!(
            "Dependency list contains id \"{}\": {}",
            id,
            self.dependency_list.contains(&id)
        );
        self.dependency_list.contains(&id)
    }

    /// Consumes and appends the given dependency list to the current one.
    pub fn append(&mut self, component_id_list: DependencyList) {
        self.dependency_list
            .append(&mut component_id_list.list().clone());
        self.dependency_list.sort();
        self.dependency_list.dedup();
    }
}

impl Display for DependencyList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.dependency_list.join(", "))
    }
}

impl From<Vec<ComponentId>> for DependencyList {
    fn from(dependency_list: Vec<ComponentId>) -> Self {
        Self { dependency_list }
    }
}

impl From<ComponentId> for DependencyList {
    fn from(value: ComponentId) -> Self {
        Self {
            dependency_list: vec![value],
        }
    }
}
