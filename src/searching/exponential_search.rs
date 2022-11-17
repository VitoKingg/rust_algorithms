use std::cmp::Ordering;

/// T(n) = O(log(n) + log(n)) = O(log(n))
/// arr must be sorted and has no bound
pub fn exponential_search<T: Ord>(arr: &[T], item: &T) -> Option<usize> {
    let arr_len = arr.len();

    if arr_len == 0 {
        return None;
    }

    let mut high = 1;
    while (high < arr_len) && (&arr[high] <= item) {
        high *= 2;
    }

    if high > arr_len {
        high = arr_len;
    }

    // binary search
    let mut low = high / 2;
    while low < high {
        let mid = low + (high - low) / 2;

        match item.cmp(&arr[mid]) {
            Ordering::Less => high = mid,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => low = mid + 1,
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_search() {
        let arr = [1, 2, 3, 4, 5];
        let item = 4;
        assert_eq!(exponential_search(&arr, &item), Some(3));

        let arr = [1, 2, 3, 4, 5];
        let item = 10;
        assert_eq!(exponential_search(&arr, &item), None);

        let arr = [];
        let item = "a";
        assert_eq!(exponential_search(&arr, &item), None);

        let arr = ["a"];
        let item = "a";
        assert_eq!(exponential_search(&arr, &item), Some(0));

        let arr = ["amazon", "facebook", "google", "microsoft"];
        let item = "google";
        assert_eq!(exponential_search(&arr, &item), Some(2));
    }
}
