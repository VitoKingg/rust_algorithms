// T(n) = O(n^2)
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }

    let mut sorted = false;
    let mut n = arr.len();

    while !sorted {
        sorted = true;

        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }

        n -= 1;
    }
}

pub fn bubble_sort2<T: Ord>(arr: &mut [T]) {
    let arr_len = arr.len();

    if arr_len < 2 {
        return;
    }

    for i in 1..arr_len {
        for j in 0..arr_len - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod sorting_tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        // unsorted
        let mut arr = vec![5, 7, 2, 3, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 7]);

        // descending
        let mut arr = vec![5, 4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // accending
        let mut arr = vec![1, 2, 3, 4, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // empty
        let mut arr: Vec<usize> = vec![];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_bubble_sort2() {
        // unsorted
        let mut arr = vec![5, 7, 2, 3, 1];
        bubble_sort2(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 7]);

        // descending
        let mut arr = vec![5, 4, 3, 2, 1];
        bubble_sort2(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // accending
        let mut arr = vec![1, 2, 3, 4, 5];
        bubble_sort2(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // empty
        let mut arr: Vec<usize> = vec![];
        bubble_sort2(&mut arr);
        assert_eq!(arr, vec![]);
    }
}
