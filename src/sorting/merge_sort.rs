/// bottom-up approach: merge sort without recursion, faster
/// T(n) = O(n * log(n))
/// S(n) = O(n)
pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    let arr_len = arr.len();

    if arr_len <= 1 {
        return;
    }

    let mut sub_array_size: usize = 1;
    while sub_array_size < arr_len {
        let mut start_index: usize = 0;

        while arr_len - start_index > sub_array_size {
            let end_index: usize = if start_index + 2 * sub_array_size > arr_len {
                arr_len
            } else {
                start_index + 2 * sub_array_size
            };

            merge(&mut arr[start_index..end_index], sub_array_size);
            start_index = end_index;
        }

        sub_array_size *= 2;
    }
}

/// top-down approach: merge sort with recursion
/// T(n) = O(n * log(n))
/// S(n) = O(n)
pub fn merge_sort_rec<T: Ord + Copy>(arr: &mut [T]) {
    let arr_len = arr.len();

    if arr_len <= 1 {
        return;
    }

    let mid = arr_len / 2;
    merge_sort_rec(&mut arr[..mid]);
    merge_sort_rec(&mut arr[mid..]);
    merge(arr, mid);
}

fn merge<T: Ord + Copy>(arr: &mut [T], mid: usize) {
    // left: left part of arr
    let left = arr[..mid].to_vec();
    // right: right part of arr
    let right = arr[mid..].to_vec();

    // i: left part index
    let mut i = 0;
    // j: right part index
    let mut j = 0;

    for elem in arr {
        if j == right.len() || (i < left.len() && left[i] < right[j]) {
            *elem = left[i];
            i += 1;
        } else {
            *elem = right[j];
            j += 1;
        }
    }
}

#[cfg(test)]
mod sorting_tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        // unsorted
        let mut arr = vec![5, 7, 2, 3, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 7]);

        let mut arr = vec![4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![-31, 0, 1, 2, 2, 4, 65, 83, 99, 782]);

        // descending
        let mut arr = vec![5, 4, 3, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // accending
        let mut arr = vec![1, 2, 3, 4, 5];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // one element
        let mut arr: Vec<usize> = vec![2];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![2]);

        // empty
        let mut arr: Vec<usize> = vec![];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![]);

        // string
        let mut arr = vec!["d", "a", "c", "b"];
        merge_sort(&mut arr);
        assert_eq!(arr, vec!["a", "b", "c", "d"]);

        let mut arr = vec!["a", "bb", "d", "cc"];
        merge_sort(&mut arr);
        assert_eq!(arr, vec!["a", "bb", "cc", "d"]);
    }

    #[test]
    fn test_merge_sort_rec() {
        // unsorted
        let mut arr = vec![5, 7, 2, 3, 1];
        merge_sort_rec(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 7]);

        let mut arr = vec![4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        merge_sort_rec(&mut arr);
        assert_eq!(arr, vec![-31, 0, 1, 2, 2, 4, 65, 83, 99, 782]);

        // descending
        let mut arr = vec![5, 4, 3, 2, 1];
        merge_sort_rec(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // accending
        let mut arr = vec![1, 2, 3, 4, 5];
        merge_sort_rec(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // one element
        let mut arr: Vec<usize> = vec![2];
        merge_sort_rec(&mut arr);
        assert_eq!(arr, vec![2]);

        // empty
        let mut arr: Vec<usize> = vec![];
        merge_sort_rec(&mut arr);
        assert_eq!(arr, vec![]);

        // string
        let mut arr = vec!["d", "a", "c", "b"];
        merge_sort_rec(&mut arr);
        assert_eq!(arr, vec!["a", "b", "c", "d"]);

        let mut arr = vec!["a", "bb", "d", "cc"];
        merge_sort(&mut arr);
        assert_eq!(arr, vec!["a", "bb", "cc", "d"]);
    }
}
