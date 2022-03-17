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
    {
      let mut map = std::collections::HashMap::new();

      $(
        map.insert($k, $v);
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
  {$($v: expr),* $(,)?} => {
    {
      let mut set = std::collections::HashSet::new();

      $(
        set.insert($v);
      )*

      set
    }
  };
}
