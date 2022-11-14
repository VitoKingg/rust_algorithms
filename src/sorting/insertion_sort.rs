// T(n) = O(n^2)
pub fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut pos = i;
        let curr = arr[i];

        while pos > 0 && curr < arr[pos - 1] {
            arr[pos] = arr[pos - 1];
            pos -= 1;
        }

        arr[pos] = curr;
    }
}

#[cfg(test)]
mod sorting_tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        // unsorted
        let mut arr = vec![5, 7, 2, 3, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 7]);

        // descending
        let mut arr = vec![5, 4, 3, 2, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // accending
        let mut arr = vec![1, 2, 3, 4, 5];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // one element
        let mut arr: Vec<usize> = vec![2];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![2]);

        // empty
        let mut arr: Vec<usize> = vec![];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![]);

        // string
        let mut arr = vec!["d", "a", "c", "b"];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec!["a", "b", "c", "d"]);
    }
}
