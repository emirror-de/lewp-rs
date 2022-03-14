![](logo/lewp-transparent-background.inkscape.png)

----------------

![Version](https://img.shields.io/crates/v/lewp?style=flat-square) [![](https://img.shields.io/docsrs/lewp?style=flat-square)](https://docs.rs/lewp) ![Downloads](https://img.shields.io/crates/d/lewp?style=flat-square) ![MIT or Apache-2.0 License](https://img.shields.io/crates/l/lewp?style=flat-square) [![](https://img.shields.io/discord/855726181142495242?color=154683&label=discord&style=flat-square)](https://discord.gg/nx7YtsjEbT)

Say goodbye to the web template hell. Generate your HTML5 website technically optimized and always valid. Without leaving Rust source.

*This is the adjusted Rust implementation of [the PHP version of lewp](https://gitlab.com/lewp/lewp).*

If you have questions, want to contribute or have any other type of request, your invited to create an issue or visit the [openprobst.dev](https://openprobst.dev) discord server.

## ❓What is lewp?

Many frameworks already exist that support developers in creating websites and -apps in various languages, eg. [Laravel](https://laravel.com/), [Symfony](https://symfony.com/), or [Django](https://www.djangoproject.com/) just to name a few. They usually follow the [MVC pattern](https://www.tutorialspoint.com/design_pattern/mvc_pattern.htm). You can create a controller for every route and add a model as well as a view for this, all in separate folders. So far, nothing new, a well known and proven concept. However, several problems can arise regarding e.g. code quality, re-usability as well as best practices of SEO and much more. The architecture of these frameworks can lead developers to become inconsistent considering their code structure. Because the files are spread over the whole project, the reusability of code can easily suffer. In addition to that, the usage of templating systems like [Twig](https://twig.symfony.com/) can make the developer create invalid HTML code without recognizing it or create problems relating to for example page loading times [(which is a crucial factor)](https://www.marketingdive.com/news/google-53-of-mobile-users-abandon-sites-that-take-over-3-seconds-to-load/426070/), the [critical rendering path](https://varvy.com/pagespeed/critical-render-path.html) or avoiding ["extra whitespace between HTML tags to avoid browser rendering quirks under some circumstances"](https://twig.symfony.com/doc/3.x/filters/spaceless.html). In the latter (a cite from the [Twig](https://twig.symfony.com/doc/2.x/filters/spaceless.html) documentation), Twig has a property to prevent this, but you need to explicitly enable it and "*its performance is directly related to the text size you are working on*". Developing in these environments can run the risk of spreading those problems over your whole project, making it almost impossible to recover from without rewriting your code base.

So the idea was to create a framework that tries to minimize the impact of the stated problems without making compromises on the comfort of programming.

Therefore, lewp focuses on

- making it **easy** for the developer in **creating valid** HTML code,
- make the developer **instantly know where** the **files are stored**,
- ***true* modularity** and therefore **re-usability** of source code,
- **minimize** the [**cumulative layout shift**](https://web.dev/cls/) on loading
- applying **SEO** best practices already in development setup as much as possible,
- **minimization** **page loading times**, especially [FCP](https://web.dev/first-contentful-paint/) and [TTI](https://web.dev/interactive/),
- **flexibility** for the developer (you are able to add other dependencies to your modules without effort),
- **reducing** the **external dependencies** that are sent to the client and therefore
- **minimize** the **chaos in** your projects **source code**

lewp generates a highly optimized HTML web page and provides automatic handling of your CSS code (and your JavaScript soon) if you want to. Have a look at the examples folder for more information about how to use it.

❗ ***lewp is not a webserver.*** It is a framework that supports you in structuring your algorithms bringing them perfectly in line with your view, without letting your code get messy! It perfectly integrates with frameworks like [rocket](https://rocket.rs) or [actix-web](https://actix.rs).

## 🥅 Project goals

1. ***Simplfying*** the creation of web pages ***without mixing programming languages*** or 
   putting logic into your HTML (like it is done in templates)

2. Creating ***modularized websites*** with ***truly isolated*** and ***reusable*** 
   components/modules, eg. automatically ***namespaced CSS*** and ***JavaScript*** (not
   implemented yet)

3. Providing a ***file hierarchy*** that is ***consistently throughout your project*** and can be ***shared*** between different projects

4. Getting the ***best of both*** worlds, ***server*** side and ***client side*** rendering

5. ***minimization*** of ***page loading*** times (especially FCP and TTI)

6. ***Removing*** initial setup ***HTML boilerplate*** code creation by the developer (the base
   skeleton with html, head and required meta, body tags are created by 
   lewp)

## 📦 Features

- [x] Build your HTML website fully from Rust source
- [x] No additional fancy markup or language, just a clean API
- [x] Never touch confusing templates again
- [x] Always serve correct, minimized HTML5
- [x] Develop your Website as fully isolated modules

## 🚌 Roadmap

- [x] Skeleton to create website with by creating the DOM
- [x] A webpage can have Modules
  - [x] Each module is surrounded by a wrapper `div`
- [x] A page is created with isolated modules (HTML only)
- [x] Modules can have Modules, infinite loops are prevented
  - [x] Submodules have `RuntimeInformation` available
- [x] `<head>` modules only
- [x] The wrapping `div` tag can be disabled *(recommended only for `<head>` modules)*
- [x] File hierarchy for CSS and JS is defined
  - [ ] Prevent `/` in IDs
  - [ ] Remove `..` in IDs
- [ ] CSS integration
  - [x] Combining files is implemented
  - [x] Minimization of CSS is implemented
  - [x] Modules without CSS are skipped
  - [x] CSS links are automatically inserted into the HTML `<head>` tag if files are available
  - [ ] CSS can be split up into "render critical" (will be inlined on rendering) and "non render critical" parts
  - [x] A CSS register is implemented that can be used as shared global querying instance for CSS files
- [x] Modules are isolated (HTML, CSS)
- [ ] Page specific CSS is possible
- [ ] JavaScript integration
  - [ ] Combining files is implemented
  - [ ] Minimization is implemented
- [ ] HTML can be streamed
- [ ] Modules are now fully isolated (HTML, CSS, JavaScript)
- [ ] JavaScript and CSS can be compiled into the binary
- [ ] Add attributes to the container wrapping the module
- [ ] Configuration using `.toml` files
  - [ ] Modules
  - [ ] Pages
- [ ] Added [html5-picture](https://github.com/emirror-de/html5-picture) support
  - [ ] Conversion of pictures on startup is possible
  - [ ] API for a global register that holds all pictures and creates the HTML code
- [ ] Provide an API for localization (l10n)
- [ ] Provide an API for generic resources
- [ ] Modules can have isolated folders for temporary files
  - [ ] CSS attributes can be configured
- [ ] Modules can send events to sub-modules

## 🤠 Contributing

Unless explicitly stated, any contribution intentionally submitted for inclusion in this project, as defined in the Apache-2.0 license, shall be dual licensed as below, without any additional terms or conditions.

Please have a look at [CONTRIBUTING.md] for guidelines and conventions.

## ⚖ License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/emirror-de/naphtha/blob/main/LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](https://github.com/emirror-de/naphtha/blob/main/LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
