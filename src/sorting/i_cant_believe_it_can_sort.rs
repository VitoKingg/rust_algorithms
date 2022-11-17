/// 代码很简单，但时间复杂度较高
pub fn i_cant_believe_it_can_sort<T: Eq + Ord + Clone>(arr: &mut [T]) {
    let arr_len = arr.len();

    if arr_len <= 1 {
        return;
    }

    for i in 0..arr_len {
        for j in 0..i {
            // 若要实现降序，则将小于号改为大于号即可
            if arr[i] < arr[j] {
                arr.swap(i, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i_cant_believe_it_can_sort() {
        // strings
        let mut arr = vec!["c", "a", "b", "d", "e"];
        i_cant_believe_it_can_sort(&mut arr);
        assert_eq!(arr, vec!["a", "b", "c", "d", "e"]);

        // unsorted
        let mut arr = vec![5, 7, 2, 3, 1];
        i_cant_believe_it_can_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 7]);

        // descending
        let mut arr = vec![5, 4, 3, 2, 1];
        i_cant_believe_it_can_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // accending
        let mut arr = vec![1, 2, 3, 4, 5];
        i_cant_believe_it_can_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // pre-sorted
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        i_cant_believe_it_can_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);

        // one element
        let mut arr = vec![1];
        i_cant_believe_it_can_sort(&mut arr);
        assert_eq!(arr, vec![1]);

        // empty
        let mut arr: Vec<usize> = vec![];
        i_cant_believe_it_can_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }
}
