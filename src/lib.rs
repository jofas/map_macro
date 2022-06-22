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

/// **Example:**
///
/// ```rust
/// use map_macro::vec_no_clone;
///
/// // atomic types do not implement the `Clone` trait
/// use std::sync::atomic::AtomicI64;
///
/// let x = vec_no_clone![AtomicI64::new(0); 10];
///
/// assert_eq!(x.len(), 10);
/// ```
///
/// ```rust
/// use map_macro::vec_no_clone;
///
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// // simply clones the reference counted pointer for each element
/// // that is not the first
/// let shared_vec = vec![Rc::new(RefCell::new(0)); 2];
///
/// {
///   let mut first = shared_vec[0].borrow_mut();
///   *first += 1;
/// }
///
/// assert_eq!(*shared_vec[0].borrow(), 1);
/// // the second element is a clone of the reference counted pointer
/// // at the first element of the vector, therefore referencing the
/// // same address in memory
/// assert_eq!(*shared_vec[1].borrow(), 1);
///
/// // the `vec_no_clone` macro does not clone the object created
/// // by the first expression but instead calls the expression for
/// // each element in the vector
/// let unshared_vec = vec_no_clone![Rc::new(RefCell::new(0)); 2];
///
/// {
///   let mut first = unshared_vec[0].borrow_mut();
///   *first += 1;
/// }
///
/// assert_eq!(*unshared_vec[0].borrow(), 1);
/// // the second element is not the same cloned reference counted
/// // pointer as it would be if it were constructed with the
/// // `vec!` macro from the standard library like it was above
/// assert_eq!(*unshared_vec[1].borrow(), 0);
/// ```
///
#[macro_export]
macro_rules! vec_no_clone {
  {$v: expr; $c: expr} => {
    $crate::vec_from_fn(|| $v, $c)
  };
}

pub fn vec_from_fn<T, F: Fn() -> T>(f: F, c: usize) -> Vec<T> {
  let mut vec = Vec::with_capacity(c);

  for _ in 0..c {
    vec.push(f());
  }

  vec
}
