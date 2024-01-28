use hashbrown::HashMap;
use std::fmt::Debug;

use map_macro::hashbrown::{hash_map, hash_map_e, hash_set};

#[derive(Debug)]
struct Dyn1;

#[derive(Debug)]
struct Dyn2;

#[test]
fn hash_map1() {
    let m = hash_map! {
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
fn hash_map2() {
    let m = hash_map! {
        0 => "a",
        1 => "b",
        2 => "c",
    };

    assert_eq!(m[&0], "a");
    assert_eq!(m[&1], "b");
    assert_eq!(m[&2], "c");
}

#[test]
fn hash_map_e1() {
    drop::<HashMap<&str, &dyn Debug>>(hash_map_e! {
        "en" => &"Hello",
        "de" => &"Hallo",
        "fr" => &"Bonjour",
        "es" => &"Hola",
    });
}

#[test]
fn hash_map_e2() {
    drop::<HashMap<&str, &dyn Debug>>(hash_map_e! {
        "1" => &Dyn1,
        "2" => &Dyn2,
    });
}

#[test]
fn hash_set1() {
    let s = hash_set! { "a", "b", "c", "d" };

    assert_eq!(s.len(), 4);

    assert!(s.contains("a"));
    assert!(s.contains("b"));
    assert!(s.contains("c"));
    assert!(s.contains("d"));

    assert!(!s.contains("e"));
}

#[test]
fn hash_set2() {
    let s = hash_set! { 0, 1, 2, 3, 0 };

    assert_eq!(s.len(), 4);

    assert!(s.contains(&0));
    assert!(s.contains(&1));
    assert!(s.contains(&2));
    assert!(s.contains(&3));

    assert!(!s.contains(&4));
}
