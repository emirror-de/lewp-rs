# CHANGELOG

## v0.4.0

### 📦 New features

* Integrated `lewp-html` version `0.3.0`
* Pages now can have head tags. The `head_tags` method has been added to the `Page` trait
* Pages now need to have method `id(&self) -> &str` implemented to identify resources that are used by that page
* Page CSS files are now added as inline style using the `css::Register`
* Added `fh::ResourceType` that is able to query resources from the file hierarchy
* Added `Text` and `Image` classes that implement `fh::Component` to enable retrieval of text and image files from the file hierarchy
* `RuntimeInformation` now inlcudes a reference to the page config instance
* `RuntimeInformation` are now passed to submodules
* `PageConfig` now contains the member `fh: Option<Arc<FileHierarchy>>` to enable access to the file hierarchy for modules
* `Lewp` struct has been introduced, enabling a shared global file hierarchy and css register
* `css::Register` has now an option to autoload and process files on instantiation which is enabled by default
* Added the css identifier `#module` that is replaced by the root node of your module on compilation. See [lewp::css](./src/css/mod.rs) for an example and detailed information.
* Added `ModuleId` and `PageId` abstractions

### 📈 Changes

* `Component` trait contains a `folder_name` method that can be used to get the correct folder in the file hierarchy based on the given `ComponentInformation`
* `PageOptions` now contains a file hierarchy and a css register as `Option`
* List returned by `FileHierarchy::get_file_list` is now sorted
* `FileHierarchy::get_file_list` now fully uses `Path` to assemble the subfolder
* Updated dependencies of the `*5ever` crates to the new versions.

### 🐛 Bugfixes

### 🔨 Breaking changes

* The `div` wrapper of the module has been removed completely
* `ModuleConfig::skip_wrapper` has been removed
* `Module::view` method now returns `Node` instead of `Nodes`
* `css_register` method has been removed from the `Page` trait, because it is stored in the `PageConfig` from now on
* `SubModule` trait has been integrated into `Module` trait
* `Nodes` has been renamed to `NodeList` as it changed in `lewp-html`

## v0.3.0

This release brings several new features.

It also adds the following crates as dependencies:

* `lewp-selectors`, a forked version of the `selectors` crate

* `lewp-css`, continued `css` crate

* `lewp-html`

### 📦 New features

* Introduced the lewp file hierarchy, see the `fh` module for more information
* A module wrapper now has the attribute `data-lewp-component` set to `module` instead of a `lewp-module` class attribute
* The `Component` trait has been added to the file hierarchy. It is now possible to have custom components that deliver content to the website.
* The `CSS` component has been added. It is now possible to store CSS files in the module folder of the file hierarchy. The files are parsed using the `lewp-css` crate.
* Only CSS files that are stored directly in the according folder are being used. There is intentionally no recursive processing to make sure the components stay small and maintainable.
* CSS files are now isolated. Every selector of the parsed CSS stylesheet gets the module id added. Therefore modules cannot interfere each others styles any longer.
* The new dependency `lewp_html` adds a clean API for creating the DOM.

### 🐛 Bugfixes

### 🔨 Breaking changes

## v0.2.0

This is the initial release of the Rust implementation of [lewp](https://gitlab.com/lewp/lewp).

### 📦 New features

* Added a basic skeleton for the generation of the DOM for building a website
* Added traits for creating a `Page` that contains `Module`s
* Modules are isolated (HTML)
* Modules can have Submodules, inifinite loops are prevented
* Modules can be `<head>`-only
* Rendering of the wrapping `div` can be disabled

### 🐛 Bugfixes

### 🔨 Breaking changes
