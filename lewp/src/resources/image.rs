use {
    crate::{
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
    std::{path::PathBuf, sync::Arc},
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
    fh: Arc<FileHierarchy>,
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

    fn content(
        &self,
        params: Self::ContentParameter,
    ) -> Result<Self::Content, LewpError> {
        let mut filename = self.folder_name();
        filename.push(params.filename);
        log::trace!("Image filename: {:#?}", filename);
        let image = match std::fs::read(&filename) {
            Ok(r) => r,
            Err(msg) => {
                return Err(LewpError::new(
                    LewpErrorKind::FileHierarchyComponent,
                    &format!(
                        "Error reading image file \"{}\" with error: {}",
                        filename.display(),
                        msg
                    ),
                    self.component_information.clone(),
                ));
            }
        };
        Ok(image)
    }

    fn file_hierarchy(&self) -> Arc<FileHierarchy> {
        self.fh.clone()
    }
}

impl Image {
    /// Creates a new Image component.
    pub fn new(
        component_information: Arc<FHComponentInformation>,
        fh: Arc<FileHierarchy>,
    ) -> Self {
        let component_information = Arc::new(FHComponentInformation {
            id: component_information.id.clone(),
            level: component_information.level,
            kind: ComponentType::Resource(ResourceType::Image),
        });
        log::trace!("ComponentInformation: {:#?}", component_information);
        Self {
            fh,
            component_information,
        }
    }
}

#[test]
fn read_rust_logo() {
    use {
        crate::{
            fh::{ComponentInformation, FileHierarchyBuilder, Level},
            resources::Image,
        },
        std::{path::PathBuf, sync::Arc},
    };

    let fh = Arc::new(
        FileHierarchyBuilder::new()
            .mountpoint(PathBuf::from("testfiles"))
            .build(),
    );
    let component_information = Arc::new(ComponentInformation {
        id: String::from("hello-world"),
        level: Level::Component,
        kind: ComponentType::Resource(ResourceType::Image),
    });
    let image_resource = Image::new(component_information, fh);
    let logo = match image_resource
        .content(ImageParameter::new("rust-logo-512x512-blk.png"))
    {
        Ok(f) => f,
        Err(e) => panic!("{:#?}", e),
    };

    assert_eq!(
        std::fs::read(
            "testfiles/components/hello-world/images/rust-logo-512x512-blk.png"
        )
        .unwrap(),
        logo
    );
}
