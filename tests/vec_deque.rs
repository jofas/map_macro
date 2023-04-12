use std::collections::VecDeque;
use std::fmt::Debug;

use map_macro::{vec_deque, vec_deque_e};

#[derive(Debug)]
struct Dyn1;

#[derive(Debug)]
struct Dyn2;

#[test]
fn vec_deque1() {
    let v = vec_deque![0, 1, 2, 3];

    assert_eq!(v[0], 0);
    assert_eq!(v[1], 1);
    assert_eq!(v[2], 2);
    assert_eq!(v[3], 3);
}

#[test]
fn vec_deque2() {
    let v = vec_deque![0; 4];

    assert_eq!(v.len(), 4);

    assert_eq!(v[0], 0);
    assert_eq!(v[1], 0);
    assert_eq!(v[2], 0);
    assert_eq!(v[3], 0);
}

#[test]
fn vec_deque_empty() {
    drop::<VecDeque<u8>>(vec_deque![]);
}

#[test]
fn vec_deque_e1() {
    drop::<VecDeque<&dyn Debug>>(vec_deque_e![&0, &"Hello"]);
}

#[test]
fn vec_deque_e2() {
    drop::<VecDeque<&dyn Debug>>(vec_deque_e![&0; 4]);
}
