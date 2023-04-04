#![ doc = include_str!("../README.md")]

/// Macro for creating a [map](std::collections::HashMap).
///
/// Equivalent to the [vec!] macro for [vectors](std::vec::Vec).
/// Set this [crate's](crate) documentation for more examples on how
/// to use this macro.
///
/// **Example:**
///
/// ```rust
/// use map_macro::map;
///
/// let goodbye = map! {
///   "en" => "Goodbye",
///   "de" => "Auf Wiedersehen",
///   "fr" => "Au revoir",
///   "es" => "Adios",
/// };
/// ```
///
#[macro_export]
macro_rules! map {
    {$($k: expr => $v: expr),* $(,)?} => {
        std::collections::HashMap::from([$(($k, $v),)*])
    };
}

/// Explicitly typed equivalent of [`map!`].
///
/// Set this [crate's](crate) documentation for more examples on how
/// to use this macro.
///
/// **Example:**
///
/// ```rust
/// use std::collections::HashMap;
/// use std::fmt::Debug;
///
/// use map_macro::map_e;
///
/// let goodbye: HashMap<&str, &dyn Debug> = map_e! {
///   "en" => &"Goodbye",
///   "de" => &"Auf Wiedersehen",
///   "fr" => &"Au revoir",
///   "es" => &"Adios",
/// };
///
/// println!("{:?}", goodbye);
/// ```
///
#[macro_export]
macro_rules! map_e {
    {$($k: expr => $v: expr),* $(,)?} => {
        std::collections::HashMap::from([$(($k, $v as _),)*])
    };
}

/// Macro for creating a [map](std::collections::BTreeMap) based on
/// a b-tree data structure.
///
/// Works just like the [map!] macro.
/// Set this [crate's](crate) documentation for more examples on how
/// to use this macro.
///
/// **Example:**
///
/// ```rust
/// use map_macro::btree_map;
///
/// let goodbye = btree_map! {
///   "en" => "Goodbye",
///   "de" => "Auf Wiedersehen",
///   "fr" => "Au revoir",
///   "es" => "Adios",
/// };
/// ```
///
#[macro_export]
macro_rules! btree_map {
    {$($k: expr => $v: expr),* $(,)?} => {
        std::collections::BTreeMap::from([$(($k, $v),)*])
    };
}

/// Explicitly typed equivalent of [`btree_map!`].
///
/// Set this [crate's](crate) documentation for more examples on how
/// to use this macro.
///
/// **Example:**
///
/// ```rust
/// use std::collections::BTreeMap;
/// use std::fmt::Debug;
///
/// use map_macro::btree_map_e;
///
/// let goodbye: BTreeMap<&str, &dyn Debug> = btree_map_e! {
///   "en" => &"Goodbye",
///   "de" => &"Auf Wiedersehen",
///   "fr" => &"Au revoir",
///   "es" => &"Adios",
/// };
/// ```
///
#[macro_export]
macro_rules! btree_map_e {
    {$($k: expr => $v: expr),* $(,)?} => {
        std::collections::BTreeMap::from([$(($k, $v as _),)*])
    };
}

/// Macro for creating a [set](std::collections::HashSet).
///
/// Equivalent to the [vec!] macro for [vectors](std::vec::Vec).
/// Set this [crate's](crate) documentation for more examples on how
/// to use this macro.
///
/// **Example:**
///
/// ```rust
/// use map_macro::set;
///
/// let x = set! { 1, 2, 3, 3, 4 };
///
/// assert_eq!(x.len(), 4);
/// ```
///
#[macro_export]
macro_rules! set {
    {$($v: expr),* $(,)?} => {
        std::collections::HashSet::from([$($v,)*])
    };
}

/// Macro for creating a [set](std::collections::BTreeSet) based on
/// a b-tree data structure.
///
/// Works just like the [set!] macro.
/// Set this [crate's](crate) documentation for more examples on how
/// to use this macro.
///
/// **Example:**
///
/// ```rust
/// use map_macro::btree_set;
///
/// let x = btree_set! { 1, 2, 3, 3, 4 };
///
/// assert_eq!(x.len(), 4);
/// ```
///
#[macro_export]
macro_rules! btree_set {
    {$($v: expr),* $(,)?} => {
        std::collections::BTreeSet::from([$($v,)*])
    };
}

/// More flexible version of the [vec!] macro.
///
/// See this [crate's](crate) documentation for a description and more
/// examples on how to use this macro.
///
/// **Example:**
///
/// ```rust
/// use map_macro::vec_no_clone;
///
/// struct UnclonableWrapper(u8);
///
/// // the `vec!` macro from the standard library would panic at this
/// // call
/// let x = vec_no_clone![UnclonableWrapper(0); 10];
///
/// assert_eq!(x.len(), 10);
/// ```
///
#[macro_export]
macro_rules! vec_no_clone {
    {$v: expr; $c: expr} => {
        {
            let mut vec = Vec::with_capacity($c);

            for _ in 0..$c {
                vec.push($v);
            }

            vec
        }
    };
    {$($v: expr),* $(,)?} => {
        {
            vec![$($v),*]
        }
    };
}
