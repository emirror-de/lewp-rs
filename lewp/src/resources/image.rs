use {
    crate::{
        archive::{Archive, ArchiveComponent},
        component::{ComponentDetails, ComponentId},
        resources::{ResourceLevel, ResourceType},
    },
    mime::Mime,
    rust_embed::RustEmbed,
    std::path::PathBuf,
};

/// The options to be passed when loading an image.
#[derive(Debug)]
pub struct ImageOptions {
    /// The component id.
    pub id: ComponentId,
    /// The resource level.
    pub level: ResourceLevel,
    /// File name to load including the extension.
    filename: PathBuf,
}

/// Enables interactions with image files in an [Archive](crate::archive::Archive).
pub struct Image {
    details: ComponentDetails,
    /// The image content.
    pub content: Vec<u8>,
}

impl ArchiveComponent for Image {
    type Options = ImageOptions;

    fn load<A: Archive>(options: Self::Options) -> anyhow::Result<Self> {
        let details = ComponentDetails::new(
            options.id.clone(),
            ResourceType::Image,
            options.level,
        );
        log::debug!("Created ComponentDetails for {options:?}:\n{details:#?}");

        let mut filename = A::path(&details);
        filename.push(options.filename);
        log::debug!("Image filename to load: {:#?}", filename);
        let filename = match filename.to_str() {
            Some(s) => s,
            None => {
                return Err(anyhow::anyhow!(
                    "Could not convert {} to str!",
                    filename.display()
                ))
            }
        };
        let image = match <A as RustEmbed>::get(&filename) {
            Some(r) => r,
            None => {
                return Err(anyhow::anyhow!(
                    "Error reading image file \"{filename}\" from file hierarchy!",
                ));
            }
        };
        Ok(Self {
            details,
            content: image.data.to_vec(),
        })
    }

    fn mime_type() -> Mime {
        mime::IMAGE_STAR
    }

    fn details(&self) -> &ComponentDetails {
        &self.details
    }
}

#[test]
fn read_rust_logo() {
    use crate::{
        lewp_archive,
        resources::{Image, WebInterface},
    };

    lewp_archive!(TestArchive, "testfiles");
    impl WebInterface for TestArchive {}

    let image_details = ImageOptions {
        id: "hello-world".into(),
        level: ResourceLevel::Component,
        filename: PathBuf::from("rust-logo-512x512-blk.png"),
    };
    let image_resource = match Image::load::<TestArchive>(image_details) {
        Ok(f) => f,
        Err(e) => panic!("{e:#?}"),
    };

    assert_eq!(
        std::fs::read(
            "testfiles/components/hello-world/images/rust-logo-512x512-blk.png"
        )
        .unwrap(),
        image_resource.content
    );
}
