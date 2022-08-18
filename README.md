# map-macro

[![Build Status](https://github.com/jofas/map_macro/actions/workflows/build.yml/badge.svg)](https://github.com/jofas/map_macro/actions/workflows/build.yml)
[![Codecov](https://codecov.io/gh/jofas/map_macro/branch/master/graph/badge.svg?token=69YKZ1JIBK)](https://codecov.io/gh/jofas/map_macro)
[![Latest Version](https://img.shields.io/crates/v/map-macro.svg)](https://crates.io/crates/map-macro)
[![Downloads](https://img.shields.io/crates/d/map-macro?label=downloads)](https://crates.io/crates/map-macro)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/map-macro/latest/map_macro)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Declarative `map!`, `set!`, `btree_map!`, `btree_set!` and
`vec_no_clone!` macros.

The `map!` macro allows for statically initializing a 
`std::collections::HashMap`.
The same goes for the `set!` macro only for 
`std::collections::HashSet`.
Both macros have an equivalent version using a b-tree data structure
rather than a hashtable-based implementation, `btree_map!` and
`btree_set!` for statically initializing `std::collections::BTreeMap`
and `std::collections::BTreeSet`, respectively.
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
   * [B-tree based maps and sets](#b-tree-based-maps-and-sets)
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


## B-tree based maps and sets 

Besides hashtable-based maps and sets, rust's standard library offers 
maps and sets based on the b-tree data structure 
(`std::collections::BTreeMap` and `std::collections::BTreeSet).
They offer similar functionality to their hashtable-based 
counterparts.
`map-macro` offers the `btree_map!` and `btree_set!` macros to 
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


## Vectors without cloning

When using the `vec![some_value; count]` syntax, the type of
`some_value` has to implement the `Clone` trait, because `some_value` 
is cloned `count - 1` times into all the vector elements, except the
first one.

This could either be undesired behavior (you don't want clones of 
`some_value`, because its type implements `Clone` in a way that 
doesn't fit your needs) or the type you wish to pre-populate your
vector with doesn't implement `Clone`.

For example, this will result in a panic during compile time:

```no_compile
struct UnclonableWrapper(u8);

// panics
let x = vec![UnclonableWrapper(0); 5];
```

The `vec_no_clone!` macro takes a different approach. 
Instead of cloning `UnclonableWrapper(0)`, it treats it as an 
expression which is called 5 times in this case.
So 5 independent `UnclonableWrapper` objects, each with its own
location in memory, are created:

```rust
use map_macro::vec_no_clone;

struct UnclonableWrapper(u8);

let x = vec_no_clone![UnclonableWrapper(0); 5];

assert_eq!(x.len(), 5);
```

Without `vec_no_clone!` you'd have to write something far less 
readable and more complex to reason about like this to create the same
vector:

```rust
struct UnclonableWrapper(u8);

let x: Vec<UnclonableWrapper> = (0..5)
  .map(|_| UnclonableWrapper(0))
  .collect();

assert_eq!(x.len(), 5);
```

`vec_no_clone!` is not only useful for types not implementing `Clone`,
but also for types where cloning them is not what you want.
The best example would be a reference counted pointer, `std::rc::Rc`.
When you clone an `Rc` instance, a new smart pointer instance 
referencing the same location in memory is created.
If you'd rather have multiple independent reference counted pointers
to different memory locations, you can use `vec_no_clone!` as well:

```rust
use map_macro::vec_no_clone;

use std::cell::RefCell;
use std::rc::Rc;

// simply clones the reference counted pointer for each element that 
// is not the first
let shared_vec = vec![Rc::new(RefCell::new(0)); 2];
{
  let mut first = shared_vec[0].borrow_mut();
  *first += 1;
}

assert_eq!(*shared_vec[0].borrow(), 1);

// the second element is a clone of the reference counted pointer at 
// the first element of the vector, referencing the same address in
// memory, therefore being mutated as well
assert_eq!(*shared_vec[1].borrow(), 1);

// the `vec_no_clone!` macro does not clone the object created by the
// first expression but instead calls the expression for each element 
// in the vector, creating two independent objects, each with their 
// own address in memory
let unshared_vec = vec_no_clone![Rc::new(RefCell::new(0)); 2];

{
  let mut first = unshared_vec[0].borrow_mut();
  *first += 1;
}

assert_eq!(*unshared_vec[0].borrow(), 1);

// the second element is not the same cloned reference counted
// pointer as it would be if it were constructed with the `vec!` macro
// from the standard library like it was above, therefore it is not
// mutated
assert_eq!(*unshared_vec[1].borrow(), 0);
```

You can also use the macro with a list of elements, like `vec!`:

```rust
use map_macro::vec_no_clone;

let v1 = vec_no_clone![0, 1, 2, 3];
let v2 = vec![0, 1, 2, 3];

assert_eq!(v1, v2);

let v1: Vec<u8> = vec_no_clone![];
let v2: Vec<u8> = vec![];

assert_eq!(v1, v2);
```
