use map_macro::map;

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
