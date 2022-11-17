#![allow(dead_code)]

pub struct BinarySearchTree<T> {
    value: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}
