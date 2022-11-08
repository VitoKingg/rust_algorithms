use rust_algorithms::sorting::{
    bubble_sort::*,
};

#[test]
fn bubble_sort_test() {
    let mut v = vec![5, 7, 2, 3, 1];

    bubble_sort(&mut v);
    assert_eq!(v, vec![1, 2, 3, 5, 7])
}
