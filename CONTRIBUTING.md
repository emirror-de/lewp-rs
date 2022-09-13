# Contributing

Thanks for choosing to contribute to `lewp-rs`! This is a summary of all information required to get accepted for a PR. If you have questions or identified missing information, please [create an issue](https://github.com/emirror-de/lewp-rs/issues/new) so they can be added.

## Code conventions

This project follows the standard [Rust naming conventions](https://rust-lang.github.io/api-guidelines/naming.html). Make sure that your code is formatted by `rustfmt` before committing.

## Branching setup

The main development takes place in the `main` branch. Please make sure that your PR goes in there. The `main` branch is used as base for the release branches.

For the different types of branches the following prefixes are used:

* `f/` for feature branches
* `b/` for bugfix branches
* `r/` for release branches
* `h/` for hotfix branches
* `v` is the version tag prefix

## Commit messages

The commit messages **must** follow [conventional commits](https://www.conventionalcommits.org/en/v1.0.0).

Available scopes are:

* `css` - for the `lewp-css` crate

* `selectors` - for `lewp-selectors` crate

* `html` - for `lewp-html` crate

* `lewp` - for the main crate

## Testing

Please make sure that every commit is a complete one, meaning the code should run as expected. This includes that the following tests

- `cargo test --all-features`

- `cargo clippy --all-features`

pass.
