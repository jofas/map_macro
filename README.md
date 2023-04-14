# map-macro

[![Build Status](https://github.com/jofas/map_macro/actions/workflows/build.yml/badge.svg)](https://github.com/jofas/map_macro/actions/workflows/build.yml)
[![Latest Version](https://img.shields.io/crates/v/map-macro.svg)](https://crates.io/crates/map-macro)
[![Downloads](https://img.shields.io/crates/d/map-macro?label=downloads)](https://crates.io/crates/map-macro)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/map-macro/latest/map_macro)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

This crate offers declarative macros for initializing collections from the 
[standard library][std].

This crate has zero dependencies.

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

## Explicitly Typed Values for Trait Objects

As shown in the example above, the compiler uses type inference to infer the correct type 
for the created map.
Unfortunately, type inference alone can not detect [trait objects][trait objects].
This will not work, because the compiler is unable to figure out the right type:

```compile_fail
use std::collections::HashMap;
use std::fmt::Debug;

use map_macro::hash_map;

let hello: HashMap<&str, &dyn Debug> = hash_map! {
    "en" => &"Hello",
    "de" => &"Hallo",
    "fr" => &"Bonjour",
    "es" => &"Hola",
};
```

The `map_e!` macro enables you to use trait objects as values through 
[type coercion][type coercion], making the example above compile successfully:

```rust
use std::collections::HashMap;
use std::fmt::Debug;

use map_macro::hash_map_e;

let hello: HashMap<&str, &dyn Debug> = hash_map_e! {
    "en" => &"Hello",
    "de" => &"Hallo",
    "fr" => &"Bonjour",
    "es" => &"Hola",
};
```

Note that you need to give an explicit type to the binding when you use `map_e!`, because 
it relies on knowing what type it should coerce the values to.

% TODO: here say something about object safety and that `_e` macros are available where
% the generic parameters are not bounded 

[std]: https://doc.rust-lang.org/std/collections/index.html
[trait objects]: https://doc.rust-lang.org/reference/types/trait-object.html
[type coercion]: https://doc.rust-lang.org/reference/type-coercions.html
[object safe]: https://doc.rust-lang.org/reference/items/traits.html#object-safety
