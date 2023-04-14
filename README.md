# map-macro

[![Build Status](https://github.com/jofas/map_macro/actions/workflows/build.yml/badge.svg)](https://github.com/jofas/map_macro/actions/workflows/build.yml)
[![Latest Version](https://img.shields.io/crates/v/map-macro.svg)](https://crates.io/crates/map-macro)
[![Downloads](https://img.shields.io/crates/d/map-macro?label=downloads)](https://crates.io/crates/map-macro)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/map-macro/latest/map_macro)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Declarative macros for initializing collections from the rust 
[standard library][std].

The `map!` macro allows for statically initializing a 
[hash map][hash map].
`set!` is does the same, only for [hash sets][hash set].
Both macros have an equivalent version using a b-tree data structure,
`btree_map!` and `btree_set!`.

The `vec_no_clone` is a more flexible version of the `vec!`
macro the standard library provides.
It allows you to create vectors with the `vec![some_value; count]` 
syntax, without cloning `some_value`.

This crate has zero dependencies.


## Table of Contents

<!--ts-->
   * [Maps](#maps)
      * [Explicitly typed values for trait objects](#explicitly-typed-values-for-trait-objects)
   * [Sets](#sets)
   * [B-tree based maps and sets](#b-tree-based-maps-and-sets)
   * [Vectors without cloning](#vectors-without-cloning)
<!--te-->


## Maps

Some languages provide neat syntactic sugar for creating non-empty 
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

In rust, creating a non-empty hash map is not as straight-forward:

```rust
use std::collections::HashMap;

let mut hello = HashMap::new();

hello.insert("en", "Hello");
hello.insert("de", "Hallo");
hello.insert("fr", "Bonjour");
hello.insert("es", "Hola");
```

More less-readable boilerplate code is needed.
Even worse, `hello` must be declared as mutable, even if we do not
want it to be mutable after we have added our four entries.
The `map-macro` crate offers a better way of declaring non-empty
maps using the `map!` macro.
Creating the same `hello` map from the example above can be simplified
to:

```rust
use map_macro::hash_map;

let hello = hash_map! {
    "en" => "Hello",
    "de" => "Hallo",
    "fr" => "Bonjour",
    "es" => "Hola",
};
```

That's it.
Looks nearly as neat as the python version, with the added benefit 
that `hello` is not mutable after we created it.

The `map!` macro is powerful enough to create maps from non-static
keys and values as well, you are not limited to literals:

```rust
use map_macro::hash_map;

fn hello_in_french() -> &'static str {
    "Bonjour"
}

fn spanish_language_code() -> &'static str {
    "es"
}

let hello = hash_map! {
    "en" => "Hello",
    "de" => "Hallo",
    "fr" => hello_in_french(),
    spanish_language_code() => "Hola",
};
```


### Explicitly typed values for trait objects

As shown in the examples above, rust uses type inference to infer
the correct type for the created hash map.
Unfortunately, type inference alone can not detect 
[trait objects][trait objects].
This will not work, because `rustc` is unable to figure out the
right type when creating `hello`:

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
[type coercion][type coercion], making the example above compile
successfully:

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

Note that you need to give an explicit type to the binding when you 
use `map_e!`, because it relies on knowing what type it should
coerce the values to.
Also, only values and not keys can be trait objects, because keys must
implement the [`Hash`][hash] trait, which is not 
[object save][object safe].

[`btree_map_e!`](#b-tree-based-maps-and-sets) is the equivalent to
`map_e!` for creating a [b-tree map][b-tree map] with trait object
values:

```rust
use std::collections::BTreeMap;
use std::fmt::Debug;

use map_macro::btree_map_e;

let hello: BTreeMap<&str, &dyn Debug> = btree_map_e! {
    "en" => &"Hello",
    "de" => &"Hallo",
    "fr" => &"Bonjour",
    "es" => &"Hola",
};
```


## Sets

Rust has the same cumbersome creation process for creating sets.

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
use map_macro::hash_set;

let x = hash_set! { 1, 2, 3 };
```

Again, nearly as neat as dart!

The `set!` macro is as powerful as the `map!` macro:

```rust
use map_macro::hash_set;

fn one() -> i32 {
  1
}

let x = hash_set! { one(), 2, 3 };
```

```rust
use std::collections::HashSet;
use map_macro::hash_set;

let x: HashSet<i32> = hash_set! {};

assert_eq!(x.len(), 0);
```


## B-tree based maps and sets 

Besides hashtable-based maps and sets, rust's standard library offers 
maps and sets based on the b-tree data structure.
They offer similar functionality to their hashtable-based 
counterparts.
`map-macro` provides the `btree_map!` and `btree_set!` macros to 
statically initialize the b-tree-based maps and sets.
They work exactly like the `map!` and `set!` macros:

```rust
use map_macro::{btree_map, btree_set};

let hello = btree_map! {
    "en" => "Hello",
    "de" => "Hallo",
    "fr" => "Bonjour",
    "es" => "Hola",
};

assert_eq!(hello["en"], "Hello");

let x = btree_set! { 1, 2, 3 };

assert!(x.contains(&1));

assert!(!x.contains(&4))
```

[std]: https://doc.rust-lang.org/std/collections/index.html
[hash map]: https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html
[hash set]: https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html
[trait objects]: https://doc.rust-lang.org/reference/types/trait-object.html
[type coercion]: https://doc.rust-lang.org/reference/type-coercions.html
[b-tree map]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
[hash]: https://doc.rust-lang.org/std/hash/trait.Hash.html
[object safe]: https://doc.rust-lang.org/reference/items/traits.html#object-safety
