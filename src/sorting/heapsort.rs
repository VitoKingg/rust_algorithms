/// T(n) = O(n * log(n))
/// unstable
pub fn heapsort<T: Ord>(arr: &mut [T]) {
    let arr_len = arr.len();

    if arr_len <= 1 {
        return;
    }

    for start in (0..=arr_len / 2).rev() {
        move_down(arr, start);
    }

    for end in (1..arr_len).rev() {
        arr.swap(0, end);
        move_down(&mut arr[..end], 0);
    }
}

/// Move the element at `root` down until `arr` is a max heap again.
///
/// This assumes that the subtrees under `root` are valid max heaps already.
fn move_down<T: Ord>(arr: &mut [T], mut root: usize) {
    let last = arr.len() - 1;

    loop {
        let left = root * 2 + 1;
        if left > last {
            break;
        }

        let right = left + 1;
        let max = if right <= last && arr[right] > arr[left] {
            right
        } else {
            left
        };

        if arr[max] > arr[root] {
            arr.swap(root, max);
        }

        root = max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heapsort() {
        // we can simply use `std::collections::BibaryHeap` in the standard library
        let arr = vec![5, 7, 2, 3, 1];
        assert_eq!(
            std::collections::BinaryHeap::from(arr).into_sorted_vec(),
            vec![1, 2, 3, 5, 7]
        );

        // unsorted
        let mut arr = vec![5, 7, 2, 3, 1];
        heapsort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 7]);

        // descending
        let mut arr = vec![5, 4, 3, 2, 1];
        heapsort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // accending
        let mut arr = vec![1, 2, 3, 4, 5];
        heapsort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // one element
        let mut arr: Vec<usize> = vec![2];
        heapsort(&mut arr);
        assert_eq!(arr, vec![2]);

        // empty
        let mut arr: Vec<usize> = vec![];
        heapsort(&mut arr);
        assert_eq!(arr, vec![]);

        // string
        let mut arr = vec!["d", "a", "c", "b"];
        heapsort(&mut arr);
        assert_eq!(arr, vec!["a", "b", "c", "d"]);
    }
}
