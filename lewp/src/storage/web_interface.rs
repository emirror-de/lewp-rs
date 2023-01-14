/// Methods that define the web interface to the storage.
pub trait WebInterface {
    /// Returns the URI path/route where the storage is mounted on the webserver.
    ///
    /// Defaults to `/resources`.
    fn uri_path() -> &'static str {
        "/resources"
    }
}
