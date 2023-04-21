use {
    crate::{
        component::ComponentId,
        resources::{ResourceId, ResourceLevel, ResourceType},
    },
    std::path::PathBuf,
};

/// Identifies a [ComponentModel](super::ComponentModel). Used to serve resources by URL.
///
/// This struct contains all required information to identify your component.
/// By combining all parameter into a URL, it is possible to serve resources
/// for a web page. See [Archive::parse](crate::archive::Archive::parse) method
/// for the default implementation.
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct ComponentDetails {
    /// The resource level.
    pub level: ResourceLevel,
    /// The resource type.
    pub resource_type: ResourceType,
    /// The component id.
    pub component_id: ComponentId,
    /// The resource id. If set to `None`, it refers to a component that does not
    /// provide individual files.
    pub resource_id: Option<ResourceId>,
}

impl ComponentDetails {
    /// Creates a new instance with the required values.
    pub fn new(
        component_id: ComponentId,
        resource_type: ResourceType,
        level: ResourceLevel,
    ) -> Self {
        Self {
            level,
            resource_type,
            component_id,
            resource_id: None,
        }
    }

    /// Consumes the instance and returns it with the given resource id.
    pub fn with_resource_id(mut self, resource_id: ResourceId) -> Self {
        self.resource_id = Some(resource_id);
        self
    }
}

impl From<ComponentDetails> for PathBuf {
    fn from(value: ComponentDetails) -> Self {
        let path = PathBuf::from(value.level.to_string())
            .join(value.component_id.to_string())
            .join(value.resource_type.to_string());
        let path = match value.resource_id {
            Some(r) => path.join(r),
            None => path,
        };
        path
    }
}

/*
impl TryFrom<PathBuf> for ComponentDetails {
    type Error = anyhow::Error;
    /// Extracts the different parts defining the link to the resource.
    ///
    /// The default implementation for a storage is:
    /// ```text
    /// resources/components/hello-world/css/my-crazy-resourcefile.ext
    /// ^         ^         ^           ^                    ^
    /// root      |         |           resource_type        |
    /// (unused)  level     component_id                 resource_id
    /// ```
    /// ## Examples
    /// ```rust
    /// # use {lewp::{resources::{ResourceLevel, ResourceId, ResourceType}, component::{ComponentId, ComponentDetails}}, std::path::PathBuf};
    /// let p = PathBuf::from("components/hello-world/css/my-crazy-resourcefile.ext");
    /// let s = ComponentDetails::try_from(p.clone()).unwrap();
    /// assert_eq!(PathBuf::from(s), p);
    ///
    /// let p = PathBuf::from("components/hello-world/css");
    /// let s = ComponentDetails::try_from(p.clone()).unwrap();
    /// assert_eq!(PathBuf::from(s), p);
    /// ```
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        // create default values
        let mut resource_id = None;

        let it = &mut value.iter().rev();
        // if the last entry has an extension, it points to a file. otherwise
        // it is specified to be a component wide resource.
        if value.extension().is_some() {
            resource_id = it.next().map(|r| ResourceId::from(r));
        }

        let resource_type = match it.next() {
            Some(r) => ResourceType::try_from(r)?,
            None => {
                return Err(anyhow::anyhow!(
                    "Could not extract resource_type from {value:?}"
                ));
            }
        };

        let component_id = match it.next() {
            Some(c) => {
                ComponentId::from(match c.to_os_string().into_string() {
                    Ok(r) => r,
                    Err(e) => return Err(anyhow::anyhow!("{e:?}")),
                })
            }
            None => {
                return Err(anyhow::anyhow!(
                    "Could not extract component_id from {value:?}"
                ));
            }
        };

        let level = match it.next() {
            Some(l) => ResourceLevel::try_from(l)?,
            None => {
                return Err(anyhow::anyhow!(
                    "Could not extract level from {value:?}"
                ));
            }
        };

        Ok(Self {
            level,
            resource_type,
            component_id,
            resource_id,
        })
    }
}
*/
