# CHANGELOG

## v0.4.0

### 📦 New features

* Integrated `lewp-html` version `0.2.0`
* Pages now can have head tags. The `head_tags` method has been added to the `Page` trait
* Pages now need to have method `id(&self) -> &str` implemented to identify resources that are used by that page
* Page CSS files are now added as inline style using the `css::Register`

### 📈 Changes

* The module `div` wrapper is now disabled by default.

### 🐛 Bugfixes

### 🔨 Breaking changes

* Renamed `skip_wrapper` to `wrapper` in `ModuleConfig`

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
