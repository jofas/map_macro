#![ doc = include_str!("../README.md")]

/// Macro for creating a [`HashMap`](std::collections::HashMap).
///
/// Syntactic sugar for [`HashMap::from`](std::collections::HashMap::from).
///
/// # Examples
///
/// ```rust
/// use map_macro::hash_map;
///
/// let goodbye = hash_map! {
///     "en" => "Goodbye",
///     "de" => "Auf Wiedersehen",
///     "fr" => "Au revoir",
///     "es" => "Adios",
/// };
/// ```
///
#[macro_export]
macro_rules! hash_map {
    {$($k: expr => $v: expr),* $(,)?} => {
        std::collections::HashMap::from([$(($k, $v),)*])
    };
}

/// Deprecated. Use [`hash_map!`] instead.
///
#[deprecated = "deprecated in favour of `hash_map!`. Will be removed in `map-macro v0.3.0`"]
#[macro_export]
macro_rules! map {
    {$($k: expr => $v: expr),* $(,)?} => {
        std::collections::HashMap::from([$(($k, $v),)*])
    };
}

/// Explicitly typed equivalent of [`hash_map!`], suitable for
/// [trait object values](crate#explicitly-typed-values-for-trait-objects).
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
/// use std::fmt::Debug;
///
/// use map_macro::hash_map_e;
///
/// let goodbye: HashMap<&str, &dyn Debug> = hash_map_e! {
///     "en" => &"Goodbye",
///     "de" => &"Auf Wiedersehen",
///     "fr" => &"Au revoir",
///     "es" => &"Adios",
/// };
///
/// println!("{:?}", goodbye);
/// ```
///
#[macro_export]
macro_rules! hash_map_e {
    {$($k: expr => $v: expr),* $(,)?} => {
        std::collections::HashMap::from([$(($k, $v as _),)*])
    };
}

/// Deprecated. Use [`hash_map_e!`] instead.
///
#[deprecated = "deprecated in favour of `hash_map_e!`. Will be removed in `map-macro v0.3.0`"]
#[macro_export]
macro_rules! map_e {
    {$($k: expr => $v: expr),* $(,)?} => {
        std::collections::HashMap::from([$(($k, $v as _),)*])
    };
}

/// Macro for creating a [`BTreeMap`](std::collections::BTreeMap).
///
/// Syntactic sugar for [`BTreeMap::from`](std::collections::BTreeMap::from).
///
/// # Examples
///
/// ```rust
/// use map_macro::btree_map;
///
/// let goodbye = btree_map! {
///     "en" => "Goodbye",
///     "de" => "Auf Wiedersehen",
///     "fr" => "Au revoir",
///     "es" => "Adios",
/// };
/// ```
///
#[macro_export]
macro_rules! btree_map {
    {$($k: expr => $v: expr),* $(,)?} => {
        std::collections::BTreeMap::from([$(($k, $v),)*])
    };
}

/// Explicitly typed equivalent of [`btree_map!`], suitable for
/// [trait object values](crate#explicitly-typed-values-for-trait-objects).
///
/// # Examples
///
/// ```rust
/// use std::collections::BTreeMap;
/// use std::fmt::Debug;
///
/// use map_macro::btree_map_e;
///
/// let goodbye: BTreeMap<&str, &dyn Debug> = btree_map_e! {
///     "en" => &"Goodbye",
///     "de" => &"Auf Wiedersehen",
///     "fr" => &"Au revoir",
///     "es" => &"Adios",
/// };
/// ```
///
#[macro_export]
macro_rules! btree_map_e {
    {$($k: expr => $v: expr),* $(,)?} => {
        std::collections::BTreeMap::from([$(($k, $v as _),)*])
    };
}

/// Macro for creating a [`HashSet`](std::collections::HashSet).
///
/// Syntactic sugar for [`HashSet::from`](std::collections::HashSet::from).
///
/// # Examples
///
/// ```rust
/// use map_macro::hash_set;
///
/// let x = hash_set! { 1, 2, 3, 3, 4 };
///
/// assert_eq!(x.len(), 4);
/// ```
///
#[macro_export]
macro_rules! hash_set {
    {$($v: expr),* $(,)?} => {
        std::collections::HashSet::from([$($v,)*])
    };
}

/// Deprecated. Use [`hash_set!`] instead.
///
#[deprecated = "deprecated in favour of `hash_set!`. Will be removed in `map-macro v0.3.0`"]
#[macro_export]
macro_rules! set {
    {$($v: expr),* $(,)?} => {
        std::collections::HashSet::from([$($v,)*])
    };
}

/// Macro for creating a [`BTreeSet`](std::collections::BTreeSet).
///
/// Syntactic sugar for [`BTreeSet::from`](std::collections::BTreeSet::from).
///
/// # Examples
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

/// Macro for creating a [`VecDeque`](std::collections::VecDeque).
///
/// Follows the same syntax as the [`vec!`] macro.
///
/// # Examples
///
/// ```
/// use map_macro::vec_deque;
///
/// let v = vec_deque![0, 1, 2, 3];
/// let v = vec_deque![0; 4];
/// ```
///
#[macro_export]
macro_rules! vec_deque {
    {$v: expr; $c: expr} => {
        {
            let mut vec = std::collections::VecDeque::with_capacity($c);

            for _ in 0..$c {
                vec.push_back($v);
            }

            vec
        }
    };
    {$($v: expr),* $(,)?} => {
        std::collections::VecDeque::from([$($v,)*])
    };
}

/// Explicitly typed equivalent of [`vec_deque!`], suitable for
/// [trait object values](crate#explicitly-typed-values-for-trait-objects).
///
/// # Examples
///
/// ```
/// use std::collections::VecDeque;
/// use std::fmt::Debug;
///
/// use map_macro::vec_deque_e;
///
/// let v: VecDeque<&dyn Debug> = vec_deque_e![&0, &1, &2, &3];
/// let v: VecDeque<&dyn Debug> = vec_deque_e![&0; 4];
/// ```
///
#[macro_export]
macro_rules! vec_deque_e {
    {$v: expr; $c: expr} => {
        {
            let mut vec = std::collections::VecDeque::with_capacity($c);

            for _ in 0..$c {
                vec.push_back($v as _);
            }

            vec
        }
    };
    {$($v: expr),* $(,)?} => {
        std::collections::VecDeque::from([$($v as _,)*])
    };
}

/// Macro for creating a [`LinkedList`](std::collections::LinkedList).
///
/// Follows the same syntax as the [`vec!`] macro.
///
/// # Examples
///
/// ```
/// use map_macro::linked_list;
///
/// let v = linked_list![0, 1, 2, 3];
/// let v = linked_list![0; 4];
/// ```
///
#[macro_export]
macro_rules! linked_list {
    {$v: expr; $c: expr} => {
        {
            let mut ll = std::collections::LinkedList::new();

            for _ in 0..$c {
                ll.push_back($v);
            }

            ll
        }
    };
    {$($v: expr),* $(,)?} => {
        std::collections::LinkedList::from([$($v,)*])
    };
}

/// Explicitly typed equivalent of [`linked_list!`], suitable for
/// [trait object values](crate#explicitly-typed-values-for-trait-objects).
///
/// # Examples
///
/// ```
/// use std::collections::LinkedList;
/// use std::fmt::Debug;
///
/// use map_macro::linked_list_e;
///
/// let v: LinkedList<&dyn Debug> = linked_list_e![&0, &1, &2, &3];
/// let v: LinkedList<&dyn Debug> = linked_list_e![&0; 4];
/// ```
///
#[macro_export]
macro_rules! linked_list_e {
    {$v: expr; $c: expr} => {
        {
            let mut ll = std::collections::LinkedList::new();

            for _ in 0..$c {
                ll.push_back($v as _);
            }

            ll
        }
    };
    {$($v: expr),* $(,)?} => {
        std::collections::LinkedList::from([$($v as _,)*])
    };
}

/// Macro for creating a [`BinaryHeap`](std::collections::BinaryHeap).
///
/// Follows the same syntax as the [`vec!`] macro.
///
/// # Examples
///
/// ```
/// use map_macro::binary_heap;
///
/// let v = binary_heap![0, 1, 2, 3];
/// let v = binary_heap![0; 4];
/// ```
///
#[macro_export]
macro_rules! binary_heap {
    {$v: expr; $c: expr} => {
        {
            let mut bh = std::collections::BinaryHeap::with_capacity($c);

            for _ in 0..$c {
                bh.push($v);
            }

            bh
        }
    };
    {$($v: expr),* $(,)?} => {
        std::collections::BinaryHeap::from([$($v,)*])
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
