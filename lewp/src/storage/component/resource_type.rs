/// The resource types that can be stored or retrieved from a storage.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    /// A text file.
    Text,
    /// A image file.
    Image,
    /// A CSS file.
    Css,
    /// A JavaScript file.
    JavaScript,
    /// A custom defined resource type.
    Custom(String),
}

impl ResourceType {
    /// Returns the extension of the given [ResourceType] if available.
    pub fn extension(&self) -> Option<String> {
        match self {
            Self::Text => Some(String::from("txt")),
            Self::Image => None,
            Self::Css => Some(String::from("css")),
            Self::JavaScript => Some(String::from("js")),
            _ => None,
        }
    }

    fn serialize(&self) -> String {
        match self {
            Self::Css => String::from("css"),
            Self::JavaScript => String::from("js"),
            Self::Text => String::from("text"),
            Self::Image => String::from("images"),
            Self::Custom(s) => {
                s.replace(' ', "-").replace(['.', '/'], "").to_lowercase()
            }
        }
    }
}

impl std::fmt::Display for ResourceType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.serialize())
    }
}
