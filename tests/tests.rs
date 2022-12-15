use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;

use map_macro::{
  btree_map, btree_map_e, btree_set, map, map_e, set, vec_no_clone,
};

#[derive(Debug)]
struct Dyn1;

#[derive(Debug)]
struct Dyn2;

#[test]
fn map1() {
  let m = map! {
    "en" => "Hello",
    "de" => "Hallo",
    "fr" => "Bonjour",
    "es" => "Hola",
  };

  assert_eq!(m["en"], "Hello");
  assert_eq!(m["de"], "Hallo");
  assert_eq!(m["fr"], "Bonjour");
  assert_eq!(m["es"], "Hola");
}

#[test]
fn map2() {
  let m = map! {
    0 => "a",
    1 => "b",
    2 => "c",
  };

  assert_eq!(m[&0], "a");
  assert_eq!(m[&1], "b");
  assert_eq!(m[&2], "c");
}

#[test]
fn map_e1() {
  let _: HashMap<&str, &dyn Debug> = map_e! {
    "en" => &"Hello",
    "de" => &"Hallo",
    "fr" => &"Bonjour",
    "es" => &"Hola",
  };
}

#[test]
fn map_e2() {
  let _: HashMap<&str, &dyn Debug> = map_e! {
    "1" => &Dyn1,
    "2" => &Dyn2,
  };
}

#[test]
fn set1() {
  let s = set! { "a", "b", "c", "d" };

  assert_eq!(s.len(), 4);

  assert!(s.contains("a"));
  assert!(s.contains("b"));
  assert!(s.contains("c"));
  assert!(s.contains("d"));

  assert!(!s.contains("e"));
}

#[test]
fn set2() {
  let s = set! { 0, 1, 2, 3, 0 };

  assert_eq!(s.len(), 4);

  assert!(s.contains(&0));
  assert!(s.contains(&1));
  assert!(s.contains(&2));
  assert!(s.contains(&3));

  assert!(!s.contains(&4));
}

#[test]
fn btree_map1() {
  let m = btree_map! {
    "en" => "Hello",
    "de" => "Hallo",
    "fr" => "Bonjour",
    "es" => "Hola",
  };

  assert_eq!(m["en"], "Hello");
  assert_eq!(m["de"], "Hallo");
  assert_eq!(m["fr"], "Bonjour");
  assert_eq!(m["es"], "Hola");
}

#[test]
fn btree_map2() {
  let m = btree_map! {
    0 => "a",
    1 => "b",
    2 => "c",
  };

  assert_eq!(m[&0], "a");
  assert_eq!(m[&1], "b");
  assert_eq!(m[&2], "c");
}

#[test]
fn btree_map_e1() {
  let _: BTreeMap<u8, &dyn Debug> = btree_map_e! {
    0 => &"a",
    1 => &"b",
    2 => &"c",
  };
}

#[test]
fn btree_map_e2() {
  let _: BTreeMap<&str, &dyn Debug> = btree_map_e! {
    "1" => &Dyn1,
    "2" => &Dyn2,
  };
}

#[test]
fn btree_set1() {
  let s = btree_set! { "a", "b", "c", "d" };

  assert_eq!(s.len(), 4);

  assert!(s.contains("a"));
  assert!(s.contains("b"));
  assert!(s.contains("c"));
  assert!(s.contains("d"));

  assert!(!s.contains("e"));
}

#[test]
fn btree_set2() {
  let s = btree_set! { 0, 1, 2, 3, 0 };

  assert_eq!(s.len(), 4);

  assert!(s.contains(&0));
  assert!(s.contains(&1));
  assert!(s.contains(&2));
  assert!(s.contains(&3));

  assert!(!s.contains(&4));
}

#[derive(PartialEq, Debug)]
struct UnclonableWrapper(i8);

#[test]
fn vec_no_clone1() {
  let v = vec_no_clone![UnclonableWrapper(0); 4];

  assert_eq!(v.len(), 4);

  assert_eq!(v[0], UnclonableWrapper(0));
  assert_eq!(v[1], UnclonableWrapper(0));
  assert_eq!(v[2], UnclonableWrapper(0));
  assert_eq!(v[3], UnclonableWrapper(0));
}

#[test]
#[allow(clippy::reversed_empty_ranges)]
fn vec_no_clone_empty() {
  let v = vec_no_clone![UnclonableWrapper(0); 0];

  assert_eq!(v.len(), 0);
}
