#![doc = include_str!("../README.md")]
//!
//! ## Explicitly Typed Macros
//!
//! As shown in the example above, the compiler uses type inference to infer the
//! correct type for the created map.
//! Unfortunately, type inference alone sometimes isn't enough.
//! For example, it can't detect [trait objects][trait objects].
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
//!     "cat" => &"Hola",
//!     "🌍" => "👋",
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
//!     "cat" => &"Hola",
//!     "🌍" => &"👋",
//! };
//! ```
//!
//! The macro above uses `as`&mdash;Rust's [casting operator]&mdash;to cast the
//! provided keys and values to the right type.
//! It pretty much desugars to:
//!
//! ```
//! use std::collections::HashMap;
//! use std::fmt::Debug;
//!
//! use map_macro::hash_map;
//!
//! let hello: HashMap<&str, &dyn Debug> = hash_map! {
//!     "en" as _ => &"Hello" as _,
//!     "de" as _ => &"Hallo" as _,
//!     "fr" as _ => &"Bonjour" as _,
//!     "es" as _ => &"Hola" as _,
//!     "cat" as _ => &"Hola" as _,
//!     "🌍" as _ => &"👋" as _,
//! };
//! ```
//!
//! This means that all kinds of casts and coercions are supported, including
//! non-capturing closures to function pointer casts:
//!
//! ```rust
//! use std::collections::HashMap;
//! use std::fmt::Debug;
//!
//! use map_macro::hash_map_e;
//!
//! let how_are_you: HashMap<&str, fn() -> &'static str> = hash_map_e! {
//!     "en" => || "How are you?",
//!     "de" => || "Wie geht's dir?",
//!     "fr" => || "Comment vas-tu?",
//!     "es" => || "¿Cómo estás?",
//!     "cat" => || "Com estàs?",
//! };
//! ```
//!
//! Or casting to convert enums to `u8`:
//!
//! ```rust
//! use std::collections::HashMap;
//! use std::fmt::Debug;
//!
//! use map_macro::hash_map_e;
//!
//! enum SupportedLocales {
//!     En,
//!     De,
//!     Fr,
//!     Es,
//!     Cat,
//! }
//!
//! // using enum to integer cast for the keys here
//! let welcome: HashMap<u8, &str> = hash_map_e! {
//!     SupportedLocales::En => "Welcome",
//!     SupportedLocales::De => "Willkommen",
//!     SupportedLocales::Fr => "Bienvenue",
//!     SupportedLocales::Es => "Bienvenido",
//!     SupportedLocales::Cat => "Benvingut",
//! };
//!
//! assert_eq!(welcome[&0], "Welcome");
//! assert_eq!(welcome[&1], "Willkommen");
//! assert_eq!(welcome[&2], "Bienvenue");
//! assert_eq!(welcome[&3], "Bienvenido");
//! assert_eq!(welcome[&4], "Benvingut");
//! ```
//!
//! Note that you need to give an explicit type to the binding when you use
//! `hash_map_e!`, because it relies on knowing what type it should coerce the
//! values to.
//!
//! The explicitly typed versions of the macros are indicated by an `_e` suffix.
//!
//! [trait objects]: https://doc.rust-lang.org/reference/types/trait-object.html
//! [type coercion]: https://doc.rust-lang.org/reference/type-coercions.html
//! [casting operator]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions
//!

#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "hashbrown")]
pub mod hashbrown;

#[cfg(feature = "std")]
mod _std;
