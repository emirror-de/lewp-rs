# Contributing

Thanks for choosing to contribute to `lewp-rs`! You can find a few details about the contributing standards that `lewp-rs` follows.

*A pull request template is currently in progress, until then please add as much as information to the PR as you can about the changes you made*

## Testing

Please make sure that every commit is a complete one, meaning the code should run as expected. This includes that all tests in `cargo test --all-features` pass.

## Commit messages

### Headline

The headline **must not** end with a period, exclamation or question mark as well as any other character that is not an alphabetic character or number. It **must** have the following structure.

`{type}: {short headline explanation (limited to 80 characters)}`

The `type` **must** be *lowercase*, the first character of the short headline *capitalized*.

The following types are allowed:

* `feat`, declares that a new feature that has been added
* `fix`, a bugfix
* `test`, everything related to tests
* `docs`, documentation updates
* `chore`, regular code maintenance

For example:
`feat: A new feature has been added`

### Body

* The commit body **must** be separated by a blank line from the headline.
* Each paragraph must be capitalized

## Code conventions

This project follows the standard [Rust naming conventions](https://rust-lang.github.io/api-guidelines/naming.html). Make sure that your code is formatted by `rustfmt` before committing.