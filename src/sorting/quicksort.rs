#![allow(dead_code)]

/// T(n) = O(n * log(n))
/// S(n) = O(log n) : Hoare partition scheme
/// S(n) = O(n) : Lomuto partition scheme
pub fn quicksort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();

    if len > 1 {
        quicksort_helper(arr, 0, len - 1);
    }
}

fn quicksort_helper<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize) {
    if low < high {
        let p = partition_hoare(arr, low, high);
        // let p = partition_lomuto(arr, low, high);

        if p > 1 {
            quicksort_helper(arr, low, p - 1);
        }
        quicksort_helper(arr, p + 1, high);
    }
}

// Hoare partition scheme
fn partition_hoare<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize) -> usize {
    // pivot can be:
    //  [
    //    the first element,
    //    the last element,
    //    the middle element,
    //    a random element,
    //    the median of three,
    //  ]
    let pivot = arr[low];
    let mut i = low;
    let mut j = high;

    loop {
        // move the left index to the left
        // while the element at the left index is less than the pivot
        while arr[i] < pivot {
            i += 1;
        }

        // move the right index to the left
        // while the element at the right index is greater than the pivot
        while arr[j] > pivot {
            j -= 1;
        }

        // if the indices crossed, return
        if i >= j {
            return j;
        }

        // swap the elements at the left and right indices
        arr.swap(i, j);
    }
}

// Lomuto partition scheme
fn partition_lomuto<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize) -> usize {
    // pivot can be:
    //  [
    //    the first element,
    //    the last element,
    //    the middle element,
    //    a random element,
    //    the median of three,
    //  ]
    let pivot = arr[high];
    let mut i = low;

    for j in low..high {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    // move the pivot element to the correct pivot position
    // (between the smaller and larger elements)
    arr.swap(i, high);

    // return the pivot index
    i
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
