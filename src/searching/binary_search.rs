use std::cmp::Ordering;

/// the array must be sorted first to be able to apply binary search
/// the sorted array may be ascending or descending
/// iterative version of binary search
/// T(n) = O(logn)
pub fn binary_search<T: Ord>(arr: &[T], item: &T) -> Option<usize> {
    let mut is_ascending = true;
    let arr_len = arr.len();
    if arr_len > 1 {
        is_ascending = arr[0] < arr[arr_len - 1];
    }

    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        // `(left + right) / 2` may overflow
        let mid = left + (right - left) / 2;

        if is_ascending {
            match item.cmp(&arr[mid]) {
                Ordering::Less => right = mid,
                Ordering::Equal => return Some(mid),
                Ordering::Greater => left = mid + 1,
            }
        } else {
            match item.cmp(&arr[mid]) {
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return Some(mid),
                Ordering::Greater => right = mid,
            }
        }
    }

    None
}

/// the array must be sorted first to be able to apply binary search
/// the sorted array may be ascending or descending
/// recursive version of binary search
pub fn binary_search_recursive<T: Ord>(
    arr: &[T],
    item: &T,
    left: &usize,
    right: &usize,
) -> Option<usize> {
    if left >= right {
        return None;
    }

    let is_ascending = arr[0] < arr[arr.len() - 1];

    // `(left + right) / 2` may overflow
    let mid = left + (right - left) / 2;

    if is_ascending {
        match item.cmp(&arr[mid]) {
            Ordering::Less => binary_search_recursive(arr, item, left, &mid),
            Ordering::Equal => Some(mid),
            Ordering::Greater => binary_search_recursive(arr, item, &(mid + 1), right),
        }
    } else {
        match item.cmp(&arr[mid]) {
            Ordering::Less => binary_search_recursive(arr, item, &(mid + 1), right),
            Ordering::Equal => Some(mid),
            Ordering::Greater => binary_search_recursive(arr, item, left, &mid),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr = vec![1, 3, 4, 6, 9];
        let item = 4;
        assert_eq!(binary_search(&arr, &item), Some(2));

        let arr = vec![12, 9, 7, 4, 1];
        let item = 4;
        assert_eq!(binary_search(&arr, &item), Some(3));

        let arr = vec![];
        let item = 4;
        assert_eq!(binary_search(&arr, &item), None);

        let arr = vec![1, 3, 4, 6, 9];
        let item = 7;
        assert_eq!(binary_search(&arr, &item), None);

        let arr = vec!["alphabet", "baidu", "facebook", "google", "microsoft"];
        let item = "google";
        assert_eq!(binary_search(&arr, &item), Some(3));
    }

    #[test]
    fn test_binary_search_recursive() {
        let arr = vec![1, 3, 4, 6, 9];
        let item = 4;
        let left = 0;
        let right = arr.len();
        assert_eq!(binary_search_recursive(&arr, &item, &left, &right), Some(2));

        let arr = vec![12, 9, 7, 4, 1];
        let item = 4;
        let left = 0;
        let right = arr.len();
        assert_eq!(binary_search_recursive(&arr, &item, &left, &right), Some(3));

        let arr = vec![];
        let item = 4;
        let left = 0;
        let right = arr.len();
        assert_eq!(binary_search_recursive(&arr, &item, &left, &right), None);

        let arr = vec![1, 3, 4, 6, 9];
        let item = 7;
        let left = 0;
        let right = arr.len();
        assert_eq!(binary_search_recursive(&arr, &item, &left, &right), None);

        let arr = vec!["alphabet", "baidu", "facebook", "google", "microsoft"];
        let item = "google";
        let left = 0;
        let right = arr.len();
        assert_eq!(binary_search_recursive(&arr, &item, &left, &right), Some(3));
    }
}
