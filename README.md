# map-macro

[![Build Status](https://github.com/jofas/map_macro/actions/workflows/build.yml/badge.svg)](https://github.com/jofas/map_macro/actions/workflows/build.yml)
[![Latest Version](https://img.shields.io/crates/v/map-macro.svg)](https://crates.io/crates/map-macro)
[![Downloads](https://img.shields.io/crates/d/map-macro?label=downloads)](https://crates.io/crates/map-macro)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/map-macro/latest/map_macro)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

This crate offers declarative macros for initializing collections from the 
[standard library][std] and [hashbrown][hashbrown].

This crate has zero dependencies and is `#![no_std]` if you opt-out of
support for the standard library collections.

## Example

```rust
use map_macro::hash_map;

let hello = hash_map! {
    "en" => "Hello",
    "de" => "Hallo",
    "fr" => "Bonjour",
    "es" => "Hola",
};
```

[std]: https://doc.rust-lang.org/std/collections/index.html
[hashbrown]: https://docs.rs/hashbrown/latest/hashbrown/
