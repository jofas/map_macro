#![doc = include_str!("../README.md")]
//!
//! ## Explicitly Typed Values for Trait Objects
//!
//! As shown in the example above, the compiler uses type inference to infer the correct type
//! for the created map.
//! Unfortunately, type inference alone can not detect [trait objects][trait objects].
//! This will not work, because the compiler is unable to figure out the right type:
//!
//! ```compile_fail
//! use std::collections::HashMap;
//! use std::fmt::Debug;
//!
//! use map_macro::hash_map;
//!
//! let hello: HashMap<&str, &dyn Debug> = hash_map! {
//!     "en" => &"Hello",
//!     "de" => &"Hallo",
//!     "fr" => &"Bonjour",
//!     "es" => &"Hola",
//! };
//! ```
//!
//! The `map_e!` macro enables you to use trait objects as values through
//! [type coercion][type coercion], making the example above compile successfully:
//!
//! ```rust
//! use std::collections::HashMap;
//! use std::fmt::Debug;
//!
//! use map_macro::hash_map_e;
//!
//! let hello: HashMap<&str, &dyn Debug> = hash_map_e! {
//!     "en" => &"Hello",
//!     "de" => &"Hallo",
//!     "fr" => &"Bonjour",
//!     "es" => &"Hola",
//! };
//! ```
//!
//! Note that you need to give an explicit type to the binding when you use `map_e!`, because
//! it relies on knowing what type it should coerce the values to.
//! Also, only values and not keys can be trait objects, because keys must
//! implement the [`Hash`][hash] trait, which is not
//! [object safe][object safe].
//!
//! Where the trait bounds on generic type parameters of the collections allow trait objects,
//! macros for explicitly typed variants are provided.
//! The explicitly typed versions of the macros are indicated by an `_e` suffix.
//!
//! [trait objects]: https://doc.rust-lang.org/reference/types/trait-object.html
//! [type coercion]: https://doc.rust-lang.org/reference/type-coercions.html
//! [object safe]: https://doc.rust-lang.org/reference/items/traits.html#object-safety
//! [hash]: https://doc.rust-lang.org/std/hash/trait.Hash.html
//!

#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "hashbrown")]
pub mod hashbrown;

#[cfg(feature = "std")]
mod _std;
