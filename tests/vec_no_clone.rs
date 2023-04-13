use map_macro::vec_no_clone;

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
