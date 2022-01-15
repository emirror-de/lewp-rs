# Contributing

Thanks for choosing to contribute to `lewp-rs`! You can find a few details about the contributing standards that `lewp-rs` follows.

*A pull request template is currently in progress, until then please add as much as information to the PR as you can about the changes you made*

## Testing

Please make sure that every commit is a complete one, meaning the code should run as expected. This includes that all tests in `cargo test --all-features` pass.

## GIT usage

This project follows [git flow](http://danielkummer.github.io/git-flow-cheatsheet/). The main development takes place in the `develop` branch. Please make sure that your PR goes in there. The `main` branch is the production release branch. The following prefixes are used:

* `f/` for feature branches
* `b/` for bugfix branches
* `r/` for release branches
* `h/` for hotfix branches
* `s/` for support branches
* `v` is the version tag prefix

## Commit messages

The commit messages **must** follow [conventional commits](https://www.conventionalcommits.org/en/v1.0.0).

Available scopes are:

* `css` - for the `lewp-css` crate

* `selectors` - for `lewp-selectors` crate

* `dom` - for `lewp-dom` crate

* `lewp` - for the main crate

## Code conventions

This project follows the standard [Rust naming conventions](https://rust-lang.github.io/api-guidelines/naming.html). Make sure that your code is formatted by `rustfmt` before committing.
