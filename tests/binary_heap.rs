use map_macro::binary_heap;

#[test]
fn binary_heap1() {
    let mut bh = binary_heap![0, 1, 2, 3];

    assert_eq!(bh.len(), 4);

    assert_eq!(bh.pop().unwrap(), 3);
    assert_eq!(bh.pop().unwrap(), 2);
    assert_eq!(bh.pop().unwrap(), 1);
    assert_eq!(bh.pop().unwrap(), 0);

    assert_eq!(bh.pop(), None);
}

#[test]
fn binary_heap2() {
    let mut bh = binary_heap![0; 4];

    assert_eq!(bh.len(), 4);

    assert_eq!(bh.pop().unwrap(), 0);
    assert_eq!(bh.pop().unwrap(), 0);
    assert_eq!(bh.pop().unwrap(), 0);
    assert_eq!(bh.pop().unwrap(), 0);

    assert_eq!(bh.pop(), None);
}
