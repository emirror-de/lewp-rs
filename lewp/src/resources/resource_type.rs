use std::ffi::OsStr;

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

impl From<&str> for ResourceType {
    fn from(value: &str) -> Self {
        match value {
            "css" => Self::Css,
            "js" => Self::JavaScript,
            "images" => Self::Image,
            "txt" => Self::Text,
            _ => Self::Custom(value.into()),
        }
    }
}

impl TryFrom<&OsStr> for ResourceType {
    type Error = anyhow::Error;
    fn try_from(value: &OsStr) -> Result<Self, Self::Error> {
        let value = match value.to_str() {
            Some(v) => v,
            None => {
                return Err(anyhow::anyhow!(
                    "Could not convert {value:?} to &str"
                ));
            }
        };
        Ok(ResourceType::from(value))
    }
}
