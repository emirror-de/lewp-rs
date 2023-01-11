use {
    crate::{
        fh::{
            Component as FHComponent,
            ComponentInformation as FHComponentInformation,
            ComponentType,
            FileHierarchy,
            ResourceType,
        },
        LewpError,
        LewpErrorKind,
    },
    std::sync::Arc,
};

/// Enables interactions with text files in the file hierarchy.
pub struct Text {
    component_information: Arc<FHComponentInformation>,
}

impl FHComponent for Text {
    /// The actual content of the text file.
    type Content = String;
    /// The file name relative to the level's "text" folder without file extension.
    type ContentParameter = String;

    fn component_information(&self) -> Arc<FHComponentInformation> {
        self.component_information.clone()
    }

    fn content<T: FileHierarchy>(
        &self,
        params: Self::ContentParameter,
    ) -> anyhow::Result<Self::Content> {
        let mut filename = T::folder(self);
        filename.push(params);
        let extension = match ComponentType::Resource(ResourceType::Text)
            .extension()
        {
            Some(e) => e,
            None => {
                return Err(anyhow::anyhow!("{}", LewpError {
                    kind: LewpErrorKind::FileHierarchyComponent,
                    message: "The extension for a text file could not be found! This error should never occur!".to_string(),
                    source_component: self.component_information(),
                }));
            }
        };
        filename.set_extension(extension);
        log::trace!("filename: {:#?}", filename);
        let text = match std::fs::read_to_string(&filename) {
            Ok(r) => r,
            Err(msg) => {
                return Err(anyhow::anyhow!(
                    "{}",
                    LewpError::new(
                        LewpErrorKind::FileHierarchyComponent,
                        &format!("Error reading text file: {msg}"),
                        self.component_information.clone(),
                    )
                ));
            }
        };
        Ok(text)
    }
}

impl Text {
    /// Creates a new Text component.
    pub fn new(component_information: Arc<FHComponentInformation>) -> Self {
        let component_information = Arc::new(FHComponentInformation {
            id: component_information.id.clone(),
            level: component_information.level,
            kind: ComponentType::Resource(ResourceType::Text),
        });
        log::trace!("ComponentInformation: {:#?}", component_information);
        Self {
            component_information,
        }
    }
}
