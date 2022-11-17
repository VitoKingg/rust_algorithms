#![allow(dead_code)]

pub struct AvlTree<T> {
    root: Option<Box<AvlNode<T>>>,
    length: usize,
}

struct AvlNode<T> {
    value: T,
    height: usize,
    left: Option<Box<AvlNode<T>>>,
    right: Option<Box<AvlNode<T>>>,
}
