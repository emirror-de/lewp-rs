# lewp-css - formerly known as css

## Prelude

This repo is a fork of https://github.com/lemonrock/css. I liked the initial work and wanted to further maintain it.

Because this crate seems to be no longer actively maintained and it was not possible for me to get a response from the author, this repository is published at crates.io under the name `lewp-css`.

## Description

lewp-css is a rust crate that implements a CSS stylesheet parser and serializer. It uses the [cssparser](https://docs.rs/crate/cssparser/) crate. It wraps source code originally forked of the style source code from [servo](https://github.com/servo/servo). The great efforts and achievements of those at Mozilla that have built Servo are very much appreciated. 

To get going add a dependency to Cargo.toml, eg:-

```toml
[dependencies]
lewp-css = "*"
```

And then add some simple code, say to `lib.rs` of the form:-

```rust
use ::lewp_css::Stylesheet;

// Parse
let stylesheetUtf8String: String = ...;
let stylesheet = Stylesheet::parse(&stylesheetUtf8String).expect("Wasn't a valid stylesheet");

// Serialize

use ::std::fmt::Write;

let mut destination: Write = ...
const include_source_urls: bool = false;
stylesheet.to_css(&mut destination, include_source_urls).expect("Wrote out a stylesheet");
```

## Purpose

I've built this crate to allow me to create Rust-ified variants of Autoprefixer and PurifyCSS. Why? Because my [cordial](https://github.com/lemonrock/cordial) web server runs facing the internet, and, given the nature of NodeJS code, it's not acceptable to have it as a dependency. It can also be used to evaluate `@media`, `@viewport` and `calc() / var() / attr()` rules and expressions.

It is, therefore, quite incomplete. I'd welcome patches to add support for parsing common (and less common) CSS properties. However, full support for parsing and calculating `calc()`, `attr()` and `var()` is present.

Please note that unlike a web browser [css] does not ignore what it does not understand. This is deliberate; it makes it far easier to catch assumptions in CSS code. Please patch the source if there's something you think should be parsed.

This crate will break compatibility regularly and often as it matures. It also pays no attention whatsoever to semver. At some point, if I persist with it, it should settle down and then be suitable for a more considered approach.

[css] is starting to move away from its origins with a different approach for `calc()`, `attr()`, support for page selectors, evaluating `attr()`, not normalising shorthand properties, and the like, all with the intention of making it easier to manipulate, optimize and serialize CSS.

## Licensing

The license for this project is Mozilla Public License 2.0 (MPL-2.0). The code forked from Servo is also licensed under Mozilla Public License 2.0. The Servo Developers, part of the Mozilla Foundation, retain copyright on the unaltered style source code, and also its implementation approach, forked from Servo.

There is a temporary fork of the selectors crate (and its dependency servo_arc), also from Servo, in src/selectors/temporary_fork (from "https://github.com/servo/servo", rev = "4f984a6428a0f497e311a0800efa55166c15aac6"). This will be removed once a newer version of the selectors crate (such as 0.19.0) is published to [crates.io](https://crates.io/).

## CSS Modifications

These are minor adjustments to CSS as it is parsed or serialized, mostly to make it more size efficient.

### @media

* aspect-ratios are normalized using Euclid's algorithm
* resolutions are reduced to either pixel densities (eg a DPI of 96 => a density of 1) or DPI from dots-per-centimetre (which is always larger)
* Keyframe percentage of '100%' is written as 'to'

### @font-face

* font feature settings have duplicated setting names removed
* font feature settings are written as '0' and '1' instead of 'on' and 'off'

### @viewport

* user-zoom values are normalized to 'zoom' and 'fixed'

## CSS Corrections and Upgrades

* The `aural` media type is converted to `speech`
* The non-standard [-webkit-device-pixel-ratio set of media queries](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/-webkit-device-pixel-ratio) are converted into `resolution` 

## TODO

### Property declarations and similar declarations

* Do not specify a dimension if the value is 0 (eg for width)
* Can omit final ';' in a list of property declarations (valid in W3C parser in any event)
* Optimise css colors to the smallest size, whether hex, rgb, or a name
* Restrict the valid range of properties that can be parsed for @page rules

### @-rules

* Optimise [@viewport](https://developer.mozilla.org/en-US/docs/Web/CSS/@viewport) by removing duplicated rules, forcing definitions to shorthand forms, etc
* Output prefixes -ms-, -o- for @viewport, and remove unsupported properties (orientation only for Opera, width and height only for IE11 / Edge)
* Support parsing @counter-style Symbol::Image()

### Media Queries

* Support Level 4 operator syntax (eg `>=`)
* Support parsing in a boolean context where a value has been omitted and defaults to zero (0) or none, eg for grid, color, etc
* Support compressing CSS by omitting default values of zero (0) or not() when serializing
