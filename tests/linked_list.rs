use std::collections::LinkedList;
use std::fmt::Debug;

use map_macro::{linked_list, linked_list_e};

#[derive(Debug)]
struct Dyn1;

#[derive(Debug)]
struct Dyn2;

#[test]
fn linked_list1() {
    let v: LinkedList<u8> = linked_list![0, 1, 2, 3];

    let mut iter = v.into_iter();

    assert_eq!(iter.next().unwrap(), 0);
    assert_eq!(iter.next().unwrap(), 1);
    assert_eq!(iter.next().unwrap(), 2);
    assert_eq!(iter.next().unwrap(), 3);
}

#[test]
fn linked_list2() {
    let v: LinkedList<u8> = linked_list![0; 4];

    assert_eq!(v.len(), 4);

    let mut iter = v.into_iter();

    assert_eq!(iter.next().unwrap(), 0);
    assert_eq!(iter.next().unwrap(), 0);
    assert_eq!(iter.next().unwrap(), 0);
    assert_eq!(iter.next().unwrap(), 0);
}

#[test]
fn linked_list_empty() {
    drop::<LinkedList<u8>>(linked_list![]);
}

#[test]
fn linked_list_e1() {
    drop::<LinkedList<&dyn Debug>>(linked_list_e![&0, &"Hello"]);
}

#[test]
fn linked_list_e2() {
    drop::<LinkedList<&dyn Debug>>(linked_list_e![&0; 4]);
}
