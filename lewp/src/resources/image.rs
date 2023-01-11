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
        let image = match std::fs::read(&filename) {
            Ok(r) => r,
            Err(msg) => {
                return Err(anyhow::anyhow!(
                    "{}",
                    LewpError::new(
                        LewpErrorKind::FileHierarchyComponent,
                        &format!(
                            "Error reading image file \"{}\" with error: {}",
                            filename.display(),
                            msg
                        ),
                        self.component_information.clone(),
                    )
                ));
            }
        };
        Ok(image)
    }
}

impl Image {
    /// Creates a new Image component.
    pub fn new(component_information: Arc<FHComponentInformation>) -> Self {
        let component_information = Arc::new(FHComponentInformation {
            id: component_information.id.clone(),
            level: component_information.level,
            kind: ComponentType::Resource(ResourceType::Image),
        });
        log::trace!("ComponentInformation: {:#?}", component_information);
        Self {
            component_information,
        }
    }
}

/*
#[test]
fn read_rust_logo() {
    use {
        crate::{
            fh::{ComponentInformation, Level},
            file_hierarchy,
            resources::Image,
        },
        std::sync::Arc,
    };

    file_hierarchy!(TestHierarchy, "testfiles");

    let component_information = Arc::new(ComponentInformation {
        id: String::from("hello-world"),
        level: Level::Component,
        kind: ComponentType::Resource(ResourceType::Image),
    });
    let image_resource = Image::new(component_information);
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
*/
