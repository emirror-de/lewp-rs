# CHANGELOG

## v0.3.0

### 📦 New features

- Added function `from_string(s: String) -> Result<Self, std::io::Error>` to `DocumentExt`, that enables to parse an HTML string into a `Document`
- Added `NodeListExt` trait
  - `from_string(s: String) -> Result<NodeList, std::io::Error>`

### 📈 Changes

- All functions in `api` module now take `impl ToString` instead of `&str`

### 🐛 Bugfixes

### 🔨 Breaking changes

* `Nodes` struct has been renamed to `NodeList` for better readability

## v0.2.0

### 📦 New features

* Added `borrow_attr` and `borrow_attrs` to `NodeExt` that are not self consuming compared to their `attr` and `attrs` equivalents

### 📈 Changes

* Internal code cleanup

### 🐛 Bugfixes

### 🔨 Breaking changes

- Changed input parameter of `img` tag method, now receiving `src: Path`, `title: &str` and `alt: &str` as required arguments.
- `hr` method does not take any arguments any longer.

## v0.1.0

Initial release.

### 📦 New features

* Added the first version of the API

### 🐛 Bugfixes

### 🔨 Breaking changes
