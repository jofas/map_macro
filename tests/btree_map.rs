use std::collections::BTreeMap;
use std::fmt::Debug;

use map_macro::{btree_map, btree_map_e};

#[derive(Debug)]
struct Dyn1;

#[derive(Debug)]
struct Dyn2;

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
    drop::<BTreeMap<u8, &dyn Debug>>(btree_map_e! {
        0 => &"a",
        1 => &"b",
        2 => &"c",
    });
}

#[test]
fn btree_map_e2() {
    drop::<BTreeMap<&str, &dyn Debug>>(btree_map_e! {
        "1" => &Dyn1,
        "2" => &Dyn2,
    });
}
