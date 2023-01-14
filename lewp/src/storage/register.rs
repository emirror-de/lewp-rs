//! Implements a resource register that is able to load resources and keep them
//! in memory.
//!
//! This is particularly useful for resources that are shared through different
//! pages or requests.

use {
    super::{ComponentInformation, FileHierarchy},
    std::{path::Path, sync::Arc},
};

/// A register that is able to keep resources in memory.
pub trait Register
where
    Self: Sized,
{
    /// The options that can be passed to the register.
    type Options;
    /// Content type that the register is about.
    type Content;
    /// Creates a new instance of the register. In here loading and initial
    /// processing of resources should be done.
    fn initialize<FH: FileHierarchy>(
        options: Self::Options,
    ) -> anyhow::Result<Self>;
    /// Queries a resource from the register. Returns `None` if the resource
    /// could not be found.
    fn query(
        &self,
        info: Arc<ComponentInformation>,
    ) -> Option<Arc<Self::Content>>;
    /// Returns a copy to the register [options](Self::Options).
    /// This method is necessary for [lewp](crate) to hot reload the register
    /// in debug mode.
    fn options(&self) -> Self::Options;
    /// The path where the resources are mounted on the webserver.
    ///
    /// For example: `/resources/css`
    fn path(&self) -> Path;
}
