use std::cmp::Ordering;

/// T(n) = O(log(log(n)))
/// arr must be sorted in ascending order
pub fn interpolation_search(arr: &[i32], item: &i32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mut low: usize = 0;
    let mut high: usize = arr.len() - 1;

    while (arr[low] != arr[high]) && (*item >= arr[low]) && (*item <= arr[high]) {
        let offset: usize =
            low + (((high - low) / (arr[high] - arr[low]) as usize) * (*item - arr[low]) as usize);

        match arr[offset].cmp(item) {
            Ordering::Less => low = offset + 1,
            Ordering::Equal => return Some(offset),
            Ordering::Greater => high = offset - 1,
        }
    }

    None
}

#[cfg(test)]
mod searching_tests {
    use super::*;

    #[test]
    fn test_interpolation_search() {
        let arr = [];
        let item = 3;
        assert_eq!(interpolation_search(&arr, &item), None);

        let arr = [1, 2, 3, 4, 5];
        let item = 10;
        assert_eq!(interpolation_search(&arr, &item), None);

        // descending order
        let arr = [5, 4, 3, 2, 1];
        let item = 2;
        assert_eq!(interpolation_search(&arr, &item), None);

        let arr = [1, 9, 10, 15, 16, 17, 19, 23, 27, 28, 29, 30, 32, 35];
        let item = 27;
        assert_eq!(interpolation_search(&arr, &item), Some(8));
    }
}
