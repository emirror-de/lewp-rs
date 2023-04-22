![](https://github.com/emirror-de/lewp-rs/raw/main/logo/lewp-transparent-background.inkscape.png)

----------------

![Version](https://img.shields.io/crates/v/lewp?style=flat-square)
[![](https://img.shields.io/docsrs/lewp?style=flat-square)](https://docs.rs/lewp)
![Downloads](https://img.shields.io/crates/d/lewp?style=flat-square)
![MIT or Apache-2.0 License](https://img.shields.io/crates/l/lewp?style=flat-square)

## ❓ What is lewp?

Lewp is a server side website rendering framework that enables you to use the
full
algorithmic power of Rust for the creation of technically valid HTML5 websites.

Its approach differs from most of the popular frameworks currently available by
not using templates, `JSX` or other custom syntax that mixes languages. This
prevents your code from getting messy ending up in unmaintainable spaghetti.

By its unique identification approach of pages, components and resources, Lewp
speeds up the development of your next website and makes it easier to maintain
even when you did not touch your source code for a longer period of time.

Thanks to lewp's file hierarchy implementation, structuring your resources has
never been easier. They even get compiled into the final binary, so you do not
need to worry about missing resources on deployment any longer. Your components
CSS and JavaScript is automatically isolated and integrated into your webpage,
so no more tearing one's hair about the optimization of your `<script>` and
`<link>` tags. You will find more simplifications and optimizations for your
website creation while exploring lewp.

Generate your HTML5 website technically optimized and always valid without
losing the algorithmic comfort and flexibility.

⚠ ***This crate is currently evolving. API breaking changes can happen anytime until v1.0.0.
Compiler warnings are currently used as development reminders and will be removed as soon as possible.***

*This is the adjusted Rust implementation of [the PHP version of lewp](https://gitlab.com/lewp/lewp).*

If you have questions, want to contribute or have any other type of request, your invited to create an issue.

## 🥅 Project goals

1. **Simplfying** the creation of web pages **without mixing programming languages** or 
   putting logic into your HTML (like it is done in templates)

2. Creating **modularized websites** with **truly isolated** and **reusable** 
   components/modules, eg. automatically **namespaced CSS** and **JavaScript**

3. Providing a default implementation of a **folder hierarchy for easy resource management** and possibility to **share between different projects**

4. Getting the **best of both** worlds, **server side rendering** and **client side application logic**

5. **Minimization** of **page loading** times (for example FCP and TTI)

6. **No HTML boilerplate** code

7. Applying **SEO** best practices already in development setup as much as possible

❗ ***lewp is not a webserver.*** It is a library that supports you in structuring your algorithms bringing them perfectly in line with your view, without letting your code get messy! It perfectly integrates with frameworks like [rocket](https://rocket.rs), [actix-web](https://actix.rs) or [axum](https://github.com/tokio-rs/axum).

## 📦 Features


- No more template hell in your code base
- No more whitespace bugs in your website
- Technically optimized, always valid, minified, HTML5 code
- Component based development, truly isolated
- Archive definition with pre-defined paths for easy resource management
- Uses [rust_embed](https://docs.rs/rust-embed/latest/rust_embed/index.html)
under the hood so all your assets are always available
- Build the DOM completely in Rust

## Quick start example

For more examples with comments have a look at the repositories
[examples](https://github.com/emirror-de/lewp-rs/tree/main/lewp/examples).

```rust
use lewp::{
    component::{Component, ComponentId, ComponentModel},
    html::{
        api::{h1, text},
        Node,
    },
    page::{Page, PageId, PageModel},
    view::PageView,
};
struct HelloWorld {
    data: String,
}
impl HelloWorld {
    pub fn new() -> Self {
        Self {
            data: String::from("Hello World!"),
        }
    }
}
impl ComponentModel for HelloWorld {
    type Message = ();
    fn id(&self) -> ComponentId {
        "hello-world".into()
    }
    fn main(&mut self) {}
    fn view(&self) -> Option<Node> {
        Some(h1(vec![text(&self.data)]))
    }
}
struct HelloWorldPage;
impl PageModel for HelloWorldPage {
    fn id(&self) -> PageId {
        "hello-world-page".into()
    }
    fn main(&self, view: &mut PageView) {
        let mut comp = Component::from(HelloWorld::new());
        view.push(&mut comp);
    }
}
fn main() {
    simple_logger::init().unwrap();
    let page = Page::from(HelloWorldPage {});
    let executed_page = page.main();
    println!("{}", executed_page.render());
}
```

## 🚌 Planned feature list

- Option to split CSS up into "render critical" (will be inlined on rendering)
and "non render critical" parts that will be inserted as `<link>`
- [html5-picture](https://github.com/emirror-de/html5-picture) support to be
able to scale the images to predefined sizes for specific breakpoint optimization
- JavaScript minification
- Provide an API for localization (l10n)
- Async main method for `PageModel` and `ComponentModel`
- More to come, throw your ideas to the issues ... :-)

## 🤠 Contributing

Unless explicitly stated, any contribution intentionally submitted for inclusion in this project, as defined in the Apache-2.0 license, shall be dual licensed as below, without any additional terms or conditions.

Please have a look at [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines and conventions.

## ⚖ License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/emirror-de/lewp-rs/blob/main/LICENSE-APACHE))
- MIT license ([LICENSE-MIT](https://github.com/emirror-de/lewp-rs/blob/main/LICENSE-MIT))

at your option.
