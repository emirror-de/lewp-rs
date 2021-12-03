/// Possible component types of the file hierarchy.
#[derive(Debug, Clone)]
pub enum ComponentType {
    /// A CSS file with `.css` extension.
    CSS,
    /// A JavaScript file with `.js` extension.
    JavaScript,
    /// A module.
    Module,
}

impl std::fmt::Display for ComponentType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        use ComponentType::*;
        let s = match self {
            CSS => String::from("css"),
            JavaScript => String::from("js"),
            Module => String::from("module"),
        };
        write!(f, "{}", s)
    }
}
