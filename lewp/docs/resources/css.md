## Using automated CSS processing

For the use of automatic `CSS` processing in your page or component, an
[ArchiveCache](crate::archive::ArchiveCache) is required.

With this properly set up, it is dead simple to add `CSS` that is isolated to
your component.
Assuming you defined your [Archive](crate::archive::Archive) like the following:
```ignore
lewp_archive!(Resources, "resources-files");
```
According to the default implementation of a lewp
[Archive](crate::archive::Archive) the `CSS` files for components are assumed to
follow this convention:
```text
CRATE_ROOT/resources-files/components/COMPONENT_ID/css/CSS_FILES.css
```
`CSS` that should affect the (whole) [Page](crate::page::Page) can be stored in:
```text
CRATE_ROOT/resources-files/pages/PAGE_ID/css/CSS_FILES.css
```
Files stored in these directories get combined and isolated on the creation of
an [ArchiveCache](crate::archive::ArchiveCache) object.

## Isolation of `CSS` files
