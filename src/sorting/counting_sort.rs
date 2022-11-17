/// T(n) = O(n + max_val)
/// S(n) = O(max_val)
pub fn counting_sort(arr: &mut [usize]) {
    let arr_len = arr.len();

    if arr_len <= 1 {
        return;
    }

    let max_val = arr.iter().max().unwrap();
    let mut count_arr = vec![0; max_val + 1];

    for &val in arr.iter() {
        count_arr[val] += 1;
    }

    let mut i = 0;
    for (val, &number) in count_arr.iter().enumerate() {
        for _ in 0..number {
            arr[i] = val;
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_sort() {
        // unsorted
        let mut arr = vec![5, 7, 2, 3, 1];
        counting_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 7]);

        // descending
        let mut arr = vec![5, 4, 3, 2, 1];
        counting_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // accending
        let mut arr = vec![1, 2, 3, 4, 5];
        counting_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // one element
        let mut arr: Vec<usize> = vec![2];
        counting_sort(&mut arr);
        assert_eq!(arr, vec![2]);

        // empty
        let mut arr: Vec<usize> = vec![];
        counting_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }
}
