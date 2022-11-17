/// T(n) = O(n * log(n))
/// S(n) = O(1)
/// 属于不稳定排序算法
pub fn comb_sort<T: Ord>(arr: &mut [T]) {
    let arr_len = arr.len();

    if arr_len <= 1 {
        return;
    }

    let mut gap = arr_len;
    let shrink = 1.3;
    let mut sorted = false;

    while !sorted {
        gap = (gap as f32 / shrink).floor() as usize;
        if gap <= 1 {
            gap = 1;
            sorted = true;
        }

        println!("{}-{}", arr_len, gap);

        for i in 0..arr_len - gap {
            let j = i + gap;
            if arr[i] > arr[j] {
                arr.swap(i, j);
                sorted = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comb_sort() {
        // unsorted
        let mut arr = vec![5, 7, 2, 3, 1];
        comb_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 7]);

        // descending
        let mut arr = vec![5, 4, 3, 2, 1];
        comb_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // accending
        let mut arr = vec![1, 2, 3, 4, 5];
        comb_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // pre-sorted
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        comb_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);

        // one element
        let mut arr = vec![1];
        comb_sort(&mut arr);
        assert_eq!(arr, vec![1]);

        // empty
        let mut arr: Vec<usize> = vec![];
        comb_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }
}
