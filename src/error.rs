use {
    crate::fh::{Component, ComponentInformation},
    std::rc::Rc,
};

/// List of error kinds that occur within lewp.
pub enum LewpErrorKind {
    /// Raised when a loop reference has been detected.
    LoopDetection,
    /// Indicates that a module has not been found.
    ModuleNotFound,
    /// Indicates an error that occured during CSS processing.
    Css,
    /// Error occured in functions of the FileHierarchy.
    FileHierarchy,
    /// Error occured in functions of a component of the FileHierarchy.
    FileHierarchyComponent,
    /// Indicates an error happened at runtime.
    Runtime,
    /// Indicates an error that occured during rendering.
    Render,
}

/// Contains the error definitions that occur in [lewp](crate).
pub struct LewpError {
    /// The error type.
    pub kind: LewpErrorKind,
    /// The error message.
    pub message: String,
    /// The component where the error has been occurred.
    pub source_component: Rc<ComponentInformation>,
}

impl From<Rc<ComponentInformation>> for LewpError {
    fn from(meta: Rc<ComponentInformation>) -> Self {
        Self {
            kind: LewpErrorKind::FileHierarchyComponent,
            message: String::from("Unspecified error occured!"),
            source_component: meta,
        }
    }
}

impl LewpError {
    /// Creates a [LewpError] instance with the given message.
    pub fn from_with_message(
        meta: Rc<ComponentInformation>,
        message: &str,
    ) -> Self {
        Self {
            kind: LewpErrorKind::FileHierarchyComponent,
            message: message.to_string(),
            source_component: meta,
        }
    }
}

impl std::fmt::Display for LewpError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        use LewpErrorKind::*;
        let kind = match self.kind {
            LoopDetection => "LOOP DETECTION",
            ModuleNotFound => "MODULE NOT FOUND",
            Css => "CSS PROCESSING",
            FileHierarchy => "FILE HIERARCHY",
            FileHierarchyComponent => "FILE HIERARCHY COMPONENT",
            Runtime => "Runtime",
            Render => "RENDER",
        };
        write!(
            f,
            "Component with id '{}', on level '{}' of type '{}' returned [{}]: {}",
            &self.source_component.id,
            &self.source_component.level,
            &self.source_component.kind,
            kind,
            self.message
            )
    }
}
