/// Macro for creating a [`HashMap`](::std::collections::HashMap).
///
/// Syntactic sugar for [`HashMap::from`](::std::collections::HashMap::from).
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
        ::std::collections::HashMap::from([$(($k, $v),)*])
    };
}

/// Deprecated. Use [`hash_map!`] instead.
///
#[deprecated = "deprecated in favour of `hash_map!`. Will be removed in `map-macro v0.3.0`"]
#[macro_export]
macro_rules! map {
    {$($k: expr => $v: expr),* $(,)?} => {
        ::std::collections::HashMap::from([$(($k, $v),)*])
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
        ::std::collections::HashMap::from([$(($k, $v as _),)*])
    };
}

/// Deprecated. Use [`hash_map_e!`] instead.
///
#[deprecated = "deprecated in favour of `hash_map_e!`. Will be removed in `map-macro v0.3.0`"]
#[macro_export]
macro_rules! map_e {
    {$($k: expr => $v: expr),* $(,)?} => {
        ::std::collections::HashMap::from([$(($k, $v as _),)*])
    };
}

/// Macro for creating a [`BTreeMap`](::std::collections::BTreeMap).
///
/// Syntactic sugar for [`BTreeMap::from`](::std::collections::BTreeMap::from).
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
        ::std::collections::BTreeMap::from([$(($k, $v),)*])
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
        ::std::collections::BTreeMap::from([$(($k, $v as _),)*])
    };
}

/// Macro for creating a [`HashSet`](::std::collections::HashSet).
///
/// Syntactic sugar for [`HashSet::from`](::std::collections::HashSet::from).
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
        ::std::collections::HashSet::from([$($v,)*])
    };
}

/// Deprecated. Use [`hash_set!`] instead.
///
#[deprecated = "deprecated in favour of `hash_set!`. Will be removed in `map-macro v0.3.0`"]
#[macro_export]
macro_rules! set {
    {$($v: expr),* $(,)?} => {
        ::std::collections::HashSet::from([$($v,)*])
    };
}

/// Macro for creating a [`BTreeSet`](::std::collections::BTreeSet).
///
/// Syntactic sugar for [`BTreeSet::from`](::std::collections::BTreeSet::from).
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
        ::std::collections::BTreeSet::from([$($v,)*])
    };
}

/// Macro for creating a [`VecDeque`](::std::collections::VecDeque).
///
/// Follows the same syntax as the [`vec!`](::std::vec!) macro.
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
            let mut vec = ::std::collections::VecDeque::with_capacity($c);

            for _ in 0..$c {
                vec.push_back($v);
            }

            vec
        }
    };
    {$($v: expr),* $(,)?} => {
        ::std::collections::VecDeque::from([$($v,)*])
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
            let mut vec = ::std::collections::VecDeque::with_capacity($c);

            for _ in 0..$c {
                vec.push_back($v as _);
            }

            vec
        }
    };
    {$($v: expr),* $(,)?} => {
        ::std::collections::VecDeque::from([$($v as _,)*])
    };
}

/// Macro for creating a [`LinkedList`](::std::collections::LinkedList).
///
/// Follows the same syntax as the [`vec!`](::std::vec!) macro.
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
            let mut ll = ::std::collections::LinkedList::new();

            for _ in 0..$c {
                ll.push_back($v);
            }

            ll
        }
    };
    {$($v: expr),* $(,)?} => {
        ::std::collections::LinkedList::from([$($v,)*])
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
            let mut ll = ::std::collections::LinkedList::new();

            for _ in 0..$c {
                ll.push_back($v as _);
            }

            ll
        }
    };
    {$($v: expr),* $(,)?} => {
        ::std::collections::LinkedList::from([$($v as _,)*])
    };
}

/// Macro for creating a [`BinaryHeap`](::std::collections::BinaryHeap).
///
/// Follows the same syntax as the [`vec!`](::std::vec!) macro.
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
            let mut bh = ::std::collections::BinaryHeap::with_capacity($c);

            for _ in 0..$c {
                bh.push($v);
            }

            bh
        }
    };
    {$($v: expr),* $(,)?} => {
        ::std::collections::BinaryHeap::from([$($v,)*])
    };
}

/// Version of the [`vec!`](::std::vec!) macro where the value does not have to implement [`Clone`].
///
/// Useful for unclonable types or where `Clone` is exerting undesired behaviour.
///
/// # Uncloneable Types
///
/// When using `vec![x; count]`, the type of `x` has to implement `Clone`, because
/// `x` is cloned `count - 1` times into all the vector elements except the first one.
/// For example, calling `vec!` will result in a panic during compile time here,
/// because `UnclonableWrapper` is not cloneable:
///
/// ```compile_fail
/// struct UnclonableWrapper(u8);
///
/// let x = vec![UnclonableWrapper(0); 5];
/// ```
///
/// The `vec_no_clone!` macro takes a different approach.
/// Instead of cloning `UnclonableWrapper(0)`, it treats it as an
/// [expression](https://doc.rust-lang.org/reference/expressions.html) which is
/// called 5 times in this case.
/// So 5 independent `UnclonableWrapper` objects, each with its own location in
/// memory, are created:
///
/// ```rust
/// use map_macro::vec_no_clone;
///
/// struct UnclonableWrapper(u8);
///
/// let x = vec_no_clone![UnclonableWrapper(0); 5];
///
/// assert_eq!(x.len(), 5);
/// ```
///
/// A real-world example where `vec_no_clone!` is a useful drop-in replacement
/// for `vec!` are [atomic types](::std::sync::atomic), which are not clonable:
///
/// ```rust
/// use std::sync::atomic::AtomicU8;
///
/// use map_macro::vec_no_clone;
///
/// let x = vec_no_clone![AtomicU8::new(0); 5];
///
/// assert_eq!(x.len(), 5);
/// ```
///
/// # Types where `Clone` exerts the wrong Behaviour
///
/// `vec_no_clone!` is not only useful for unclonable types, but also for types
/// where cloning them is not what you want.
/// The best example would be a reference counted pointer [`Rc`](::std::rc::Rc).
/// When you clone an `Rc`, a new instance referencing the same location in memory
/// is created.
/// If you'd rather have multiple independent reference counted pointers to
/// different memory locations, you can use `vec_no_clone!` as well:
///
/// ```rust
/// use map_macro::vec_no_clone;
///
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// // simply clones the reference counted pointer for each element that
/// // is not the first
/// let shared_vec = vec![Rc::new(RefCell::new(0)); 2];
/// {
///     let mut first = shared_vec[0].borrow_mut();
///     *first += 1;
/// }
///
/// assert_eq!(*shared_vec[0].borrow(), 1);
///
/// // the second element is a clone of the reference counted pointer at
/// // the first element of the vector, referencing the same address in
/// // memory, therefore being mutated as well
/// assert_eq!(*shared_vec[1].borrow(), 1);
///
/// // the `vec_no_clone!` macro does not clone the object created by the
/// // first expression but instead calls the expression for each element
/// // in the vector, creating two independent objects, each with their
/// // own address in memory
/// let unshared_vec = vec_no_clone![Rc::new(RefCell::new(0)); 2];
///
/// {
///     let mut first = unshared_vec[0].borrow_mut();
///     *first += 1;
/// }
///
/// assert_eq!(*unshared_vec[0].borrow(), 1);
///
/// // the second element is not the same cloned reference counted
/// // pointer as it would be if it were constructed with the `vec!` macro
/// // from the standard library like it was above, therefore it is not
/// // mutated
/// assert_eq!(*unshared_vec[1].borrow(), 0);
/// ```
///
/// # Drawbacks of using Expressions
///
/// Since `vec_no_clone!` treats the value as an expression, you must provide the
/// initialization as input directly.
/// This, for example, won't work:
///
/// ```compile_fail
/// use map_macro::vec_no_clone;
///
/// struct UnclonableWrapper(u8);
///
/// let a = UnclonableWrapper(0);
///
/// // a will have moved into the first element of x, raising a compile
/// // time error for the second element.
/// let x = vec_no_clone![a; 5];
/// ```
///
/// # Processing Lists of Elements
///
/// You can also use the macro with a list of elements, like `vec!`.
/// In fact, `vec_no_clone!` falls back to `vec!` in this case:
///
/// ```rust
/// use map_macro::vec_no_clone;
///
/// let v1 = vec_no_clone![0, 1, 2, 3];
/// let v2 = vec![0, 1, 2, 3];
///
/// assert_eq!(v1, v2);
///
/// let v1: Vec<u8> = vec_no_clone![];
/// let v2: Vec<u8> = vec![];
///
/// assert_eq!(v1, v2);
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
