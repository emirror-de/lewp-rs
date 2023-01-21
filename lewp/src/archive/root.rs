use std::path::PathBuf;

/// Defines the root on the file system to the archive.
pub trait ArchiveRoot {
    /// Returns the path to the archive.
    fn root() -> PathBuf {
        PathBuf::from("resources")
    }
}
