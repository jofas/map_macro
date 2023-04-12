use map_macro::btree_set;

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
