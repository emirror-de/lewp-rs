# CHANGELOG

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
