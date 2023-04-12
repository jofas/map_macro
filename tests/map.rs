use std::collections::HashMap;
use std::fmt::Debug;

use map_macro::{map, map_e};

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
    drop::<HashMap<&str, &dyn Debug>>(map_e! {
        "en" => &"Hello",
        "de" => &"Hallo",
        "fr" => &"Bonjour",
        "es" => &"Hola",
    });
}

#[test]
fn map_e2() {
    drop::<HashMap<&str, &dyn Debug>>(map_e! {
        "1" => &Dyn1,
        "2" => &Dyn2,
    });
}
