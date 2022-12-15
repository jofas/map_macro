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
  (@to_unit $($_:tt)*) => (());
  (@count $($tail:expr),*) => (
    <[()]>::len(&[$(map!(@to_unit $tail)),*])
  );

  {$($k: expr => $v: expr),* $(,)?} => {
    {
      let mut map = std::collections::HashMap::with_capacity(
        map!(@count $($k),*),
      );

      $(
        map.insert($k, $v);
      )*

      map
    }
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
  (@to_unit $($_:tt)*) => (());
  (@count $($tail:expr),*) => (
    <[()]>::len(&[$(map_e!(@to_unit $tail)),*])
  );

  {$($k: expr => $v: expr),* $(,)?} => {
    {
      let mut map = std::collections::HashMap::with_capacity(
        map_e!(@count $($k),*),
      );

      $(
        map.insert($k, $v as _);
      )*

      map
    }
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
    {
      let mut map = std::collections::BTreeMap::new();

      $(
        map.insert($k, $v);
      )*

      map
    }
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
    {
      let mut map = std::collections::BTreeMap::new();

      $(
        map.insert($k, $v as _);
      )*

      map
    }
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
  (@to_unit $($_:tt)*) => (());
  (@count $($tail:expr),*) => (
    <[()]>::len(&[$(set!(@to_unit $tail)),*])
  );

  {$($v: expr),* $(,)?} => {
    {
      let mut set = std::collections::HashSet::with_capacity(
        set!(@count $($v),*),
      );

      $(
        set.insert($v);
      )*

      set
    }
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
    {
      let mut set = std::collections::BTreeSet::new();

      $(
        set.insert($v);
      )*

      set
    }
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
