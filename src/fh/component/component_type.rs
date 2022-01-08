/// Possible component types of the file hierarchy.
#[derive(Debug, Clone)]
pub enum ComponentType {
    /// This represents a core component.
    Core(String),
    /// A CSS file with `.css` extension.
    Css,
    /// A JavaScript file with `.js` extension.
    JavaScript,
    /// A module.
    Module,
    /// A custom defined component. The String attached is also used as folder
    /// name, therefore all whitespaces are replaced by hyphens and everything
    /// is converted to lowercase.
    Plugin(String),
}

impl ComponentType {
    /// Returns the extension of the given [ComponentType] if available.
    pub fn extension(&self) -> Option<String> {
        match self {
            Self::Css => Some(String::from("css")),
            Self::JavaScript => Some(String::from("js")),
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
            Plugin(s) => s.replace(" ", "-").to_lowercase(),
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
