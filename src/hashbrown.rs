//! Macros for initializing [`hashbrown`] maps and sets.
//!
//! # Example
//!
//! ```
//! use map_macro::hashbrown::hash_map;
//!
//! let hello = hash_map! {
//!     "en" => "Hello",
//!     "de" => "Hallo",
//!     "fr" => "Bonjour",
//!     "es" => "Hola",
//!     "cat" => "Hola",
//!     "🌍" => "👋",
//! };
//! ```
//!
//! # Supported Versions of `hashbrown`
//!
//! As of writing this, up to the current `hashbrown` version `0.14` **all**
//! versions of `hashbrown` are supported.
//! So you can use the macros from this module with any version of `hashbrown`
//! to date.
//! Though highly likely, compatibility can't be guaranteed with future versions
//! of `hashbrown` that break [SemVer compatibility](https://semver.org/#semantic-versioning-specification-semver)
//! with `0.14`.
//! If `hashbrown` were to remove the [`FromIterator`](::core::iter::FromIterator)
//! implementations of `HashMap` and `HashSet` in a release that is
//! incompatible with `0.14` (i.e. `0.15` or `1.0`) compatibility with the
//! macros from this module would break for that new version.
//!
//! **Note:** to be compatible with all versions of `hashbrown` at once, this
//! crate doesn't re-export `hashbrown`.
//! That means that (I) you need to specify it as a dependency yourself and
//! (II) you can't rename it or the macros from this module won't be able to
//! import the needed types, resulting in a compile-time error.
//!

/// Macro for creating a [`HashMap`](::hashbrown::HashMap).
///
/// Syntactic sugar for [`HashMap::from_iter`](::hashbrown::HashMap#method.from_iter).
///
/// # Examples
///
/// ```rust
/// use map_macro::hashbrown::hash_map;
///
/// let goodbye = hash_map! {
///     "en" => "Goodbye",
///     "de" => "Auf Wiedersehen",
///     "fr" => "Au revoir",
///     "es" => "Adios",
///     "cat" => "Adéu",
/// };
/// ```
///
#[doc(hidden)]
#[macro_export]
macro_rules! __hb_hash_map {
    {$($k: expr => $v: expr),* $(,)?} => {
        <::hashbrown::HashMap::<_, _> as ::core::iter::FromIterator<_>>::from_iter([$(($k, $v),)*])
    };
}

/// Explicitly typed equivalent of [`hash_map!`](self::hash_map).
///
/// See the [explicity typed macros](crate#explicitly-typed-macros) section.
///
/// # Examples
///
/// ```rust
/// use std::fmt::Debug;
///
/// use hashbrown::HashMap;
///
/// use map_macro::hashbrown::hash_map_e;
///
/// let goodbye: HashMap<&str, &dyn Debug> = hash_map_e! {
///     "en" => &"Goodbye",
///     "de" => &"Auf Wiedersehen",
///     "fr" => &"Au revoir",
///     "es" => &"Adios",
///     "cat" => &"Adéu",
/// };
///
/// println!("{:?}", goodbye);
/// ```
///
#[doc(hidden)]
#[macro_export]
macro_rules! __hb_hash_map_e {
    {$($k: expr => $v: expr),* $(,)?} => {
        <::hashbrown::HashMap::<_, _> as ::core::iter::FromIterator<_>>::from_iter([$(($k as _, $v as _),)*])
    };
}

/// Macro for creating a [`HashSet`](::hashbrown::HashSet).
///
/// Syntactic sugar for [`HashSet::from_iter`](::hashbrown::HashSet#method.from_iter).
///
/// # Examples
///
/// ```rust
/// use map_macro::hashbrown::hash_set;
///
/// let x = hash_set! { 1, 2, 3, 3, 4 };
///
/// assert_eq!(x.len(), 4);
/// ```
///
#[doc(hidden)]
#[macro_export]
macro_rules! __hb_hash_set {
    {$($v: expr),* $(,)?} => {
        <::hashbrown::HashSet::<_> as ::core::iter::FromIterator<_>>::from_iter([$($v,)*])
    };
}

/// Explicitly typed equivalent of [`hash_set!`](self::hash_set).
///
/// See the [explicity typed macros](crate#explicitly-typed-macros) section.
///
/// # Examples
///
/// ```rust
/// use hashbrown::HashSet;
///
/// use map_macro::hashbrown::hash_set_e;
///
/// enum Foo { A, B, C, D }
///
/// let x: HashSet<u8> = hash_set_e! { Foo::A, Foo::B, Foo::C, Foo::C, Foo::D };
///
/// assert_eq!(x.len(), 4);
/// ```
///
#[doc(hidden)]
#[macro_export]
macro_rules! __hb_hash_set_e {
    {$($v: expr),* $(,)?} => {
        <::hashbrown::HashSet::<_> as ::core::iter::FromIterator<_>>::from_iter([$($v as _,)*])
    };
}

#[doc(inline)]
pub use __hb_hash_map as hash_map;

#[doc(inline)]
pub use __hb_hash_map_e as hash_map_e;

#[doc(inline)]
pub use __hb_hash_set as hash_set;

#[doc(inline)]
pub use __hb_hash_set_e as hash_set_e;
