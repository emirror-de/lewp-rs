![](logo/lewp-transparent-background.inkscape.png)

----------------

![Version](https://img.shields.io/crates/v/lewp?style=flat-square) [![](https://img.shields.io/docsrs/lewp?style=flat-square)](https://docs.rs/lewp) ![Downloads](https://img.shields.io/crates/d/lewp?style=flat-square) ![MIT or Apache-2.0 License](https://img.shields.io/crates/l/lewp?style=flat-square)

## ❓What is lewp?

Lewp is a modular library that supports you in generating and rendering
your website with ease. Your components will be automatically isolated so
CSS and JavaScript definitions are not a pain anymore and do not interfere each other!
It also provides you with the possibility to
manage different types of resources like images required for your website and
embeds them on release build into the final binary.
Lewp also saves you from getting stuck and lost in the web template hell
by **NOT** mixing languages as other solutions do.

Generate your HTML5 website technically optimized and always valid without
losing the algorithmic comfort and flexibility.

⚠ ***This crate is currently evolving. API breaking changes can happen anytime until v1.0.0.
Compiler warnings are currently used as development reminders and will be removed as soon as possible.***

*This is the adjusted Rust implementation of [the PHP version of lewp](https://gitlab.com/lewp/lewp).*

If you have questions, want to contribute or have any other type of request, your invited to create an issue.

## 🥅 Project goals

1. ***Simplfying*** the creation of web pages ***without mixing programming languages*** or 
   putting logic into your HTML (like it is done in templates)

2. Creating ***modularized websites*** with ***truly isolated*** and ***reusable*** 
   components/modules, eg. automatically ***namespaced CSS*** and ***JavaScript***

3. Providing a ***storage with pre-defined folder hierarchy*** for easy resource management and possibility to ***share*** between different projects

4. Getting the ***best of both*** worlds, ***server side rendering*** and ***client side application logic***

5. ***Minimization*** of ***page loading*** times (for example FCP and TTI)

6. ***No HTML boilerplate*** code

7. Applying **SEO** best practices already in development setup as much as possible

❗ ***lewp is not a webserver.*** It is a library that supports you in structuring your algorithms bringing them perfectly in line with your view, without letting your code get messy! It perfectly integrates with frameworks like [rocket](https://rocket.rs), [actix-web](https://actix.rs) or [axum](https://github.com/tokio-rs/axum).

## 📦 Features


- [x] No more template hell in your code base
- [x] No more whitespace bugs in your website
- [x] Technically optimized, always valid, minified, HTML5 code
- [x] Component based development, truly isolated with minimum overhead
- [x] Storage definition with pre-defined paths for easy resource management
- [x] Uses [rust_embed](https://docs.rs/rust-embed/latest/rust_embed/index.html)
under the hood so all your assets are always available
- [x] Build the DOM completely in Rust

## 🚌 Planned feature list

- [ ] CSS can be split up into "render critical" (will be inlined on rendering)
and "non render critical" parts that will be inserted as `<link>`
- [ ] [html5-picture](https://github.com/emirror-de/html5-picture) support to be
able to scale the images to predefined sizes for specific breakpoint optimization
- [ ] JavaScript *per component* isolation
- [ ] Provide an API for localization (l10n)

## 🤠 Contributing

Unless explicitly stated, any contribution intentionally submitted for inclusion in this project, as defined in the Apache-2.0 license, shall be dual licensed as below, without any additional terms or conditions.

Please have a look at [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines and conventions.

## ⚖ License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/emirror-de/naphtha/blob/main/LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](https://github.com/emirror-de/naphtha/blob/main/LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
