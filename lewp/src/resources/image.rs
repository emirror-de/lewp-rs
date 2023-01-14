use {
    crate::{
        component::ComponentId,
        storage::{Level, ResourceType, Storage, StorageComponent},
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
    id: ComponentId,
    level: Level,
}

impl StorageComponent for Image {
    /// The actual content of the image.
    type Content = Vec<u8>;
    /// The image parameters.
    type ContentParameter = ImageParameter;

    fn content<T: Storage>(
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
                    "Error reading image file \"{filename}\" from file hierarchy!",
                ));
            }
        };
        Ok(image.data.to_vec())
    }

    fn id(&self) -> ComponentId {
        self.id.clone()
    }

    fn level(&self) -> Level {
        self.level
    }

    fn kind(&self) -> ResourceType {
        ResourceType::Image
    }
}

impl Image {
    /// Creates a new Image component.
    pub fn new(id: ComponentId, level: Level) -> Self {
        Self { id, level }
    }
}

#[test]
fn read_rust_logo() {
    use crate::{lewp_storage, resources::Image, storage::Level};

    lewp_storage!(TestStorage, "testfiles");

    let image_resource =
        Image::new(ComponentId::from("hello-world"), Level::Component);
    let logo = match image_resource.content::<TestStorage>(ImageParameter::new(
        "rust-logo-512x512-blk.png",
    )) {
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
