# What is an Archive?

An [Archive] is the place where your resources live. This can be anything from
`JavaScript` or `CSS` files to your required media. It also creates a bridge
between your `lewp` webpage generation as well as the webserver you are using.
This is due to [Archive] having

* [ArchiveRoot], defining the folder on your file system, and
* [WebInterface], defining the route where the archive is mounted on the webserver,

as trait bounds.

Please have a look at the
[archive example](https://github.com/emirror-de/lewp-rs/blob/main/lewp/examples/archive.rs)
for more details on how to use it.

In general, all you need to do is to use the [lewp_archive]
macro and implement the required traits on it, for example:

```rust
# use {
#     lewp::{
#         lewp_archive,
#         archive::{
#             ArchiveRoot,
#         },
#         resources::WebInterface,
#     },
#     std::path::PathBuf,
# };
// This defines where your archive is stored.
// You can have multiple depending on your use case.
// The given path (second argument) is relative to your crate root.
// ArchiveRoot trait is automatically implemented by the macro.
lewp_archive!(ResourceArchive, "testfiles");
// WebInterface trait defaults to "/resources", so change it if your
// webserver provides the files at another location
impl WebInterface for ResourceArchive {
	fn web_root() -> PathBuf {
		PathBuf::from("/testfiles")
	}
}
```

At the current stage of development it is assumed that your webserver is also
written in Rust and has access to your archive struct.

# Serving resources from a webserver

It is pretty simple to serve your resources from a custom webserver. Keeping
the example for the `ResourceArchive` above the implementation of a handler for
eg `axum` should look like the following:
```rust
# use {
#     axum::{
#         http::{header, HeaderMap, StatusCode, Uri},
#         response::IntoResponse,
#     },
#     lewp::{
#         lewp_archive,
#         archive::{Archive, ArchiveCache, ArchiveComponent, ArchiveRoot},
#         resources::{Css, Js, ResourceType, WebInterface},
#     },
#     std::{path::PathBuf, sync::Arc},
# };
# lewp_archive!(ResourceArchive, "testfiles");
# impl WebInterface for ResourceArchive {
# 	fn web_root() -> PathBuf {
# 		PathBuf::from("/testfiles")
# 	}
# }
pub async fn resources(
    file: Uri,
    archive_cache: Arc<ArchiveCache>,
) -> impl IntoResponse {
    // Extract the component details from Uri. You can adjust the Uri to your
    // requirements by overriding the Archive::parse method.
    let details = match ResourceArchive::parse(PathBuf::from(file.path())) {
        Ok(s) => s,
        Err(e) => {
            log::warn!("Requested resource could not be parsed: {e}");
            return (StatusCode::NOT_FOUND).into_response();
        }
    };
    // we take `resource_id` here as well to always make sure a specific resource
    // has been chosen
    match (&details.resource_id, &details.resource_type) {
        (Some(_), _) => {
            // We serve anything that is not covered below directly as a file.
            let file_path = ResourceArchive::path(&details);
            let file_path = match file_path.to_str() {
                Some(f) => f,
                None => return (StatusCode::NOT_FOUND).into_response(),
            };
            match ResourceArchive::get(file_path) {
                Some(p) => p.data.into_response(),
                None => (StatusCode::NOT_FOUND).into_response(),
            }
        }
        (None, ResourceType::Css) => {
            // In case of CSS that is processed by lewp, we can query the component
            // from the archive. Lewp will be able to inline render critical
            // CSS and query anything else from an archive.
            let css = match archive_cache.query::<Css>(&details) {
                Some(c) => c,
                None => {
                    return (StatusCode::NOT_FOUND).into_response();
                }
            };
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                Css::mime_type().to_string().parse().unwrap(),
            );
            (headers, (*css.content.full).clone()).into_response()
        }
        (None, ResourceType::JavaScript) => {
            // JavaScript is also processed by lewp and stored as component.
            let js = match archive_cache.query::<Js>(&details) {
                Some(c) => c,
                None => {
                    return (StatusCode::NOT_FOUND).into_response();
                }
            };
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                Js::mime_type().to_string().parse().unwrap(),
            );
            (headers, (*js.content).clone()).into_response()
        }
        _ => (StatusCode::NOT_FOUND).into_response(),
    }
}
```

# Efficiently serving resources

To efficiently process and serve resources, `lewp` provides the [ArchiveCache]
struct. This cache implementation loads all resources on instantiation into
memory and keeps them until the restart of your server. This is especially useful
when having resources that require processing before they can be used. This can
either be for example images that are scaled to different sizes, or `CSS` that
is split up into parts (as `lewp` does).
