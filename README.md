![https://gitlab.com/lewp/lewp/-/raw/master/logo/lewp-transparent-background.inkscape.svg](https://gitlab.com/lewp/lewp/-/raw/master/logo/lewp-transparent-background.inkscape.svg)

----------------

![Version](https://img.shields.io/crates/v/lewp?style=flat-square)![https://docs.rs/lewp](https://img.shields.io/docsrs/lewp?style=flat-square)![Downloads](https://img.shields.io/crates/d/lewp?style=flat-square)![MIT or Apache-2.0 License](https://img.shields.io/crates/l/lewp?style=flat-square)[![img](https://camo.githubusercontent.com/1fc0bfbdd255ff207d4a467fb67d5e00cc3b54e2b904771c95ae7e98cc8a5798/68747470733a2f2f696d672e736869656c64732e696f2f646973636f72642f3835353732363138313134323439353234323f636f6c6f723d313534363833266c6162656c3d646973636f7264267374796c653d666c61742d737175617265)](https://camo.githubusercontent.com/1fc0bfbdd255ff207d4a467fb67d5e00cc3b54e2b904771c95ae7e98cc8a5798/68747470733a2f2f696d672e736869656c64732e696f2f646973636f72642f3835353732363138313134323439353234323f636f6c6f723d313534363833266c6162656c3d646973636f7264267374796c653d666c61742d737175617265)

Say goodbye to the web template hell. Generate your HTML5 website technically optimized and always valid. In your Rust source.

*This is the adjusted Rust implementation of [the PHP version of lewp](https://gitlab.com/lewp/lewp).*

If you have questions, want to contribute or have any other type of request, your invited to create an issue or visit the [openprobst.dev](https://openprobst.dev) discord server.

## Features

- [x] Build your HTML website fully from Rust source
- [x] Never touch confusing templates again
- [x] Always emit correct, minimized HTML5
- [x] Develop your Website as fully self-contained modules, never interfering each other

## Roadmap

- [x] Skeleton to create website with by creating the DOM
- [x] A webpage can have Modules
    - [x] Each module is surrounded by a wrapper `div`
- [x] A page is created with isolated modules (HTML only)
- [x] Modules can have Modules, infinite loops are prevented
    - [x] Submodules have `RuntimeInformation` available
- [x] `<head>` modules only
- [x] The wrapping `div` tag can be disabled *(recommended only for `<head>` modules)*
- [ ] Add possibility to add attributes to the container wrapping the module
- [ ] Added CSS integration
    - [ ] File structure is defined
    - [ ] Combining files is implemented
    - [ ] Minimization of CSS is implemented
    - [ ] Directories can be configured on module level
- [ ] Modules are isolated (HTML, CSS)
- [ ] Added [html5-picture](https://github.com/emirror-de/html5-picture) support
    - [ ] Conversion of pictures on startup is possible
    - [ ] API for a global register that holds all pictures and creates the HTML code
- [ ] Added JavaScript integration
    - [ ] File structure is defined
    - [ ] Combining files is implemented
    - [ ] Minimization is implemented
    - [ ] JavaScript is isolated
    - [ ] Directories can be configured on module level
- [ ] HTML can be streamed
- [ ] Modules are now fully isolated (HTML, CSS, JavaScript)
- [ ] CSS can be split up into "render critical" (will be inlined on rendering) and "non render critical" parts
    - [ ] CSS attributes can be configured
- [ ] JavaScript and CSS can be added to the binary
- [ ] Modules can send events to sub-modules

## Contributing

Unless explicitly stated, any contribution intentionally submitted for inclusion in this project, as defined in the Apache-2.0 license, shall be dual licensed as below, without any additional terms or conditions.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/emirror-de/naphtha/blob/main/LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](https://github.com/emirror-de/naphtha/blob/main/LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
