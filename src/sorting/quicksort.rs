/// T(n) = O(n * log(n))
pub fn quicksort<T: Ord + std::fmt::Debug>(arr: &mut [T]) {
    let arr_len = arr.len();
    if arr_len <= 1 {
        return;
    }

    _quicksort(arr, 0, arr_len - 1);
}

fn _quicksort<T: Ord + std::fmt::Debug>(arr: &mut [T], left: usize, right: usize) {
    if left < right {
        let part = partition(arr, left, right);

        // boundary check: `part <= 1` leads to `part-1 <= 0`
        if part > 1 {
            _quicksort(arr, left, part - 1);
        }
        _quicksort(arr, part + 1, right);
    }
}

fn partition<T: Ord>(arr: &mut [T], left: usize, right: usize) -> usize {
    // pivot can be:
    //  [
    //    the first element,
    //    the last element,
    //    the middle element,
    //    a random element,
    //    the median of three
    //  ]
    let pivot = left;
    // i: left marker
    let mut i = left;
    // j: right marker
    let mut j = right;

    loop {
        while i <= j && arr[i] <= arr[pivot] {
            i += 1;
        }

        while i <= j && arr[j] >= arr[pivot] {
            j -= 1;
        }

        if i > j {
            break;
        } else {
            arr.swap(i, j);
        }
    }

    arr.swap(pivot, j);
    j
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quicksort() {
        // unsorted
        let mut arr = vec![5, 7, 2, 3, 1];
        quicksort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 7]);

        // descending
        let mut arr = vec![5, 4, 3, 2, 1];
        quicksort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // accending
        let mut arr = vec![1, 2, 3, 4, 5];
        quicksort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // pre-sorted
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        quicksort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);

        // one element
        let mut arr = vec![1];
        quicksort(&mut arr);
        assert_eq!(arr, vec![1]);

        // empty
        let mut arr: Vec<usize> = vec![];
        quicksort(&mut arr);
        assert_eq!(arr, vec![]);
    }
}
