/// T(n) = O(n^2)
pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let arr_len = arr.len();

    for left in 0..arr_len {
        let mut smallest = left;

        for right in (left + 1)..arr_len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }

        arr.swap(smallest, left);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        // unsorted
        let mut arr = vec![5, 7, 2, 3, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 7]);

        // descending
        let mut arr = vec![5, 4, 3, 2, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // accending
        let mut arr = vec![1, 2, 3, 4, 5];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // one element
        let mut arr: Vec<usize> = vec![2];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![2]);

        // empty
        let mut arr: Vec<usize> = vec![];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![]);

        // string
        let mut arr = vec!["d", "a", "c", "b"];
        selection_sort(&mut arr);
        assert_eq!(arr, vec!["a", "b", "c", "d"]);
    }
}
