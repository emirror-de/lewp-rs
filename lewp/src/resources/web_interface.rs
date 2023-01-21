use std::path::PathBuf;

/// Required methods when a resource should be available from a webserver.
pub trait WebInterface {
    /// Defines the root path where resources are available on the webserver.
    ///
    /// Defaults to `/resources`
    fn web_root() -> PathBuf {
        PathBuf::from("/resources")
    }
}
