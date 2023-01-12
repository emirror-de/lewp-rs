use {
    crate::{
        component::ComponentId,
        fh::{
            Component as FHComponent,
            ComponentInformation as FHComponentInformation,
            ComponentType,
            FileHierarchy,
            Level,
            ResourceType,
        },
        LewpError,
        LewpErrorKind,
    },
    rust_embed::RustEmbed,
    std::sync::Arc,
};

/// The parameter required for the image
pub struct ImageParameter {
    /// The file name relative to the levels "images" directory including the
    /// extension.
    filename: String,
}

impl ImageParameter {
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
        }
    }
}

/// Enables interactions with image files in the file hierarchy.
pub struct Image {
    component_information: Arc<FHComponentInformation>,
}

impl FHComponent for Image {
    /// The actual content of the image.
    type Content = Vec<u8>;
    /// The image parameters.
    type ContentParameter = ImageParameter;

    fn component_information(&self) -> Arc<FHComponentInformation> {
        self.component_information.clone()
    }

    fn content<T: FileHierarchy>(
        &self,
        params: Self::ContentParameter,
    ) -> anyhow::Result<Self::Content> {
        let mut filename = T::folder(self);
        filename.push(params.filename);
        log::trace!("Image filename: {:#?}", filename);
        let filename = match filename.to_str() {
            Some(s) => s,
            None => {
                return Err(anyhow::anyhow!(
                    "Could not convert {} to str!",
                    filename.display()
                ))
            }
        };
        let image = match <T as RustEmbed>::get(&filename) {
            Some(r) => r,
            None => {
                return Err(anyhow::anyhow!(
                    "{}",
                    LewpError::new(
                        LewpErrorKind::FileHierarchyComponent,
                        &format!(
                            "Error reading image file \"{}\" from file hierarchy!",
                            filename,
                        ),
                        self.component_information.clone(),
                    )
                ));
            }
        };
        Ok(image.data.to_vec())
    }
}

impl Image {
    /// Creates a new Image component.
    pub fn new(id: ComponentId, level: Level) -> Self {
        let component_information = Arc::new(FHComponentInformation {
            id,
            level,
            kind: ComponentType::Resource(ResourceType::Image),
        });
        log::trace!("ComponentInformation: {:#?}", component_information);
        Self {
            component_information,
        }
    }
}

#[test]
fn read_rust_logo() {
    use crate::{fh::Level, file_hierarchy, resources::Image};

    file_hierarchy!(TestHierarchy, "testfiles");

    let image_resource =
        Image::new(ComponentId::from("hello-world"), Level::Component);
    let logo = match image_resource.content::<TestHierarchy>(
        ImageParameter::new("rust-logo-512x512-blk.png"),
    ) {
        Ok(f) => f,
        Err(e) => panic!("{e:#?}"),
    };

    assert_eq!(
        std::fs::read(
            "testfiles/components/hello-world/images/rust-logo-512x512-blk.png"
        )
        .unwrap(),
        logo
    );
}
