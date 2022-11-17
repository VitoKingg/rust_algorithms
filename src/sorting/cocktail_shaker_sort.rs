/// T(n) = O(n)
pub fn cocktail_shaker_sort<T: Ord>(arr: &mut [T]) {
    let arr_len = arr.len();

    if arr_len <= 1 {
        return;
    }

    loop {
        let mut swapped = false;

        for i in 0..(arr_len - 1).clamp(0, arr_len) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        swapped = false;

        for i in (0..(arr_len - 1).clamp(0, arr_len)).rev() {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cocktail_shaker_sort() {
        // unsorted
        let mut arr = vec![5, 7, 2, 3, 1];
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 7]);

        // descending
        let mut arr = vec![5, 4, 3, 2, 1];
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // accending
        let mut arr = vec![1, 2, 3, 4, 5];
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // pre-sorted
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);

        // one element
        let mut arr = vec![1];
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![1]);

        // empty
        let mut arr: Vec<usize> = vec![];
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }
}
