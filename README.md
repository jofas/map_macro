# map-macro

[![Build Status](https://github.com/jofas/map_macro/actions/workflows/build.yml/badge.svg)](https://github.com/jofas/map_macro/actions/workflows/build.yml)
[![Codecov](https://codecov.io/gh/jofas/map_macro/branch/master/graph/badge.svg?token=69YKZ1JIBK)](https://codecov.io/gh/jofas/map_macro)
[![Latest Version](https://img.shields.io/crates/v/map-macro.svg)](https://crates.io/crates/map-macro)
[![Downloads](https://img.shields.io/crates/d/map-macro?label=downloads)](https://crates.io/crates/map-macro)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/map-macro/latest/map_macro)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Declarative `map!`, `set!` and `vec_no_clone!` macros.

The `map!` macro allows for statically initializing a 
`std::collections::HashMap`.
The same goes for the `set!` macro only for 
`std::collections::HashSet`.
The macros are equivalent to the `vec!` macro from the rust standard
library.

The `vec_no_clone` is a more flexible version of the `vec!`
macro the standard library provides.
It allows you to create vectors with the `vec![some_value; count]`,
without cloning `some_value`.

This crate has zero dependencies.


## Table of Contents

<!--ts-->
   * [Maps](#maps)
   * [Sets](#sets)
   * [Vectors without cloning](#vectors-without-cloning)
<!--te-->


## Maps

Some languages provide a neat way for creating non-empty 
maps/dictionaries.
For example, in python you can create a non-empty map by running the
following code:

```python
hello = {
  "en": "Hello",
  "de": "Hallo",
  "fr": "Bonjour",
  "es": "Hola",
}
```

In rust, creating a non-empty map (rust has a built-in type in the
standard library for creating hash maps `std::collections::HashMap`)
is not as straight-forward:

```rust
use std::collections::HashMap;

let mut hello = HashMap::new();

hello.insert("en", "Hello");
hello.insert("de", "Hallo");
hello.insert("fr", "Bonjour");
hello.insert("es", "Hola");
```

More less-readable boilerplate code is needed in rust to create a
non-empty map.
Even worse, `hello` must be declared as mutable, even if we do not
want it to be mutable after we have added our four entries.
The `map-macro` crate offers a better way of declaring non-empty
maps, with the `map!` macro.
Creating the same `hello` map from the example above can be simplified
to:

```rust
use map_macro::map;

let hello = map! {
  "en" => "Hello",
  "de" => "Hallo",
  "fr" => "Bonjour",
  "es" => "Hola",
};
```

That is it.
Looks nearly as neat as the python version with the added benefit 
that `hello` is not mutable after we have created it.

The `map!` macro is powerful enough to create maps from non-static
keys and values as well, you are not limited to literals.
You can create a map like this:

```rust
use map_macro::map;

fn hello_in_french() -> &'static str {
  "Bonjour"
}

fn spanish_language_code() -> &'static str {
  "es"
}

let hello = map! {
  "en" => "Hello",
  "de" => "Hallo",
  "fr" => hello_in_french(),
  spanish_language_code() => "Hola",
};
```

Empty maps can be created as well, but must provide type hints for the
compiler:

```rust
use std::collections::HashMap;
use map_macro::map;

let hello: HashMap<&str, &str> = map! {};

assert_eq!(hello.len(), 0);
```


## Sets

Rust has the same cumbersome creation process for creating sets (in 
rust sets are provided by the standard library, too, via the 
`std::collections::HashSet` struct).

In python you can create a set like this:

```python
x = set([1, 2, 3])
```

Not as neat as a map, but still quite concise. 
Dart even comes with syntactic sugar for creating a set:

```dart
final x = {1, 2, 3};
```

In rust, you would have to write:

```rust
use std::collections::HashSet;

let mut x = HashSet::new();

x.insert(1);
x.insert(2);
x.insert(3);
```

The `set!` macro provided by the `map-macro` crate lets you write the
same code as:

```
use map_macro::set;

let x = set! { 1, 2, 3 };
```

Again, nearly as neat as dart!

The `set!` macro is as powerful as the `map!` macro:

```rust
use map_macro::set;

fn one() -> i32 {
  1
}

let x = set! { one(), 2, 3 };
```

```rust
use std::collections::HashSet;
use map_macro::set;

let x: HashSet<i32> = set! {};

assert_eq!(x.len(), 0);
```


## Vectors without cloning

TODO
