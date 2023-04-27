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

## Using `Archive` or `ArchiveCache`

While it is recommended to use an [ArchiveCache] instance for your resources,
you can also use [Archive] to have more fine control about your resources.

### `Archive`

To use the [Archive] trait with your resource you
simply make use of the [ArchiveComponent] trait that you implemented for your
resource. If you are unsure how the content of this trait should look like, have
a look at the [Js](crate::resources::Js) component implementation for reference.

To use a resource you can follow this simple example:
```rust
use {
    lewp::{
        lewp_archive,
        resources::{Resource, Js, JsOptions, ResourceLevel, WebInterface},
    },
};
lewp_archive!(ResourceArchive, "testfiles");
impl WebInterface for ResourceArchive {}

// these options depend on the resource implementation
let options = JsOptions {
    id: "hello-world".into(),
    level: ResourceLevel::Component,
};
// loads the resource from disk
let js = Resource::<Js>::load::<ResourceArchive>(options).unwrap();
```

### `ArchiveCache`

To efficiently process and serve resources, `lewp` provides the [ArchiveCache]
struct. This cache implementation loads all resources on instantiation into
memory and keeps them until the restart of your server.
Using [ArchiveCache] is the recommended way to use resources in your app since
it is faster due to its memory cache. Resources that are added to the [ArchiveCache]
are loaded upon instantiation. This means the best way to use it is when you spin
up your server. [ArchiveCache] is especially useful when having resources that
require processing before they can be used. This can either be for example
images that are scaled to different sizes, or `CSS` that
is split up into parts (as `lewp` does).

So your `main.rs` could look like the following:
```rust
use {
    lewp::{
        lewp_archive,
        archive::ArchiveCache,
        resources::{Resource, Js, JsOptions, ResourceLevel, WebInterface},
    },
};
lewp_archive!(ResourceArchive, "testfiles");
impl WebInterface for ResourceArchive {}

fn main() {
    // initialize whatever is required for your app...

    let archive_cache = ArchiveCache::default()
        .load_css::<ResourceArchive>().unwrap()
        .load_javascript::<ResourceArchive>().unwrap()
        // you can add more resources by using the insert or insert_all method
        .seal(); // seal wraps the ArchiveCache into an Arc so it is read-only

    // ... spin up your server
}
```

## Route handler implementation

It is pretty simple to serve your resources from a custom webserver. Keeping
the example for the `ResourceArchive` above the implementation of a handler for
eg `axum` could look like the following:
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

## Adding `ArchiveCache` to a page

[Page](crate::page::Page)s need to know if they should serve your webpage in combination with an
[ArchiveCache]. So for `lewp` to be able to automatically add resources like
`CSS` or `JavaScript` you need to provide this information to the page:

```rust
# #[derive(Default)]
# struct HomePage;
# 
# impl PageModel for HomePage {
#     fn id(&self) -> PageId {
#         "home-page".into()
#     }
#     fn main(&self, view: &mut PageView) {}
# }
# use {
#     axum::{
#         response::Html,
#     },
#     lewp::{
#         page::{PageModel, PageId, Page},
#         view::PageView,
#         archive::{ArchiveCache},
#         resources::WebInterface,
#     },
#     std::sync::Arc,
# };
fn your_route_handler(archive_cache: Arc<ArchiveCache>) -> Html<String> {
    let page = Page::from(HomePage::default())
        .with_archive_cache(archive_cache);
    let page = page.main();
    Html(page.render())
}
```
