use super::ResourceType;

/// Possible component types of the file hierarchy.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComponentType {
    /// This represents a core component.
    Core(String),
    /// A CSS file with `.css` extension.
    Css,
    /// A JavaScript file with `.js` extension.
    JavaScript,
    /// A module.
    Module,
    /// Resources that are stored in the file hierarchy.
    Resource(ResourceType),
    /// A custom defined component. The String attached is also used as folder
    /// name, therefore all whitespaces are replaced by hyphens, all dots and
    /// slashes are removed and everything is converted to lowercase.
    Plugin(String),
}

impl ComponentType {
    /// Returns the extension of the given [ComponentType] if available.
    pub fn extension(&self) -> Option<String> {
        match self {
            Self::Css => Some(String::from("css")),
            Self::JavaScript => Some(String::from("js")),
            Self::Resource(t) => match t {
                ResourceType::Text => Some(String::from("txt")),
                ResourceType::Image => None,
            },
            _ => None,
        }
    }

    fn serialize(&self) -> String {
        use ComponentType::*;
        match self {
            Core(s) => format!("core-{}", s),
            Css => String::from("css"),
            JavaScript => String::from("js"),
            Module => String::from("module"),
            Resource(t) => match t {
                ResourceType::Text => String::from("text"),
                ResourceType::Image => String::from("images"),
            },
            Plugin(s) => s
                .replace(" ", "-")
                .replace(".", "")
                .replace("/", "")
                .to_lowercase(),
        }
    }
}

impl std::fmt::Display for ComponentType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.serialize())
    }
}
