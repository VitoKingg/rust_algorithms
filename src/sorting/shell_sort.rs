// T(n) = O()
pub fn shell_sort<T: Ord + Copy>(arr: &mut [T]) {
    let mut gap = arr.len() / 2;
    while gap > 0 {
        for start in 0..gap {
            insertion_sort(arr, start, gap);
        }
        gap /= 2;
    }
}

fn insertion_sort<T: Ord + Copy>(arr: &mut [T], start: usize, gap: usize) {
    let mut i = start + gap;

    while i < arr.len() {
        let mut pos = i;
        let curr = arr[pos];

        while pos >= gap && curr < arr[pos - gap] {
            arr[pos] = arr[pos - gap];
            pos -= gap;
        }

        arr[pos] = curr;
        i += gap;
    }
}

#[cfg(test)]
mod sorting_tests {
    use super::*;

    #[test]
    fn test_shell_sort() {
        // unsorted
        let mut arr = vec![5, 7, 2, 3, 1];
        shell_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 7]);

        let mut arr = vec![4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        shell_sort(&mut arr);
        assert_eq!(arr, vec![-31, 0, 1, 2, 2, 4, 65, 83, 99, 782]);

        // descending
        let mut arr = vec![5, 4, 3, 2, 1];
        shell_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // accending
        let mut arr = vec![1, 2, 3, 4, 5];
        shell_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        // one element
        let mut arr: Vec<usize> = vec![2];
        shell_sort(&mut arr);
        assert_eq!(arr, vec![2]);

        // empty
        let mut arr: Vec<usize> = vec![];
        shell_sort(&mut arr);
        assert_eq!(arr, vec![]);

        // string
        let mut arr = vec!["d", "a", "c", "b"];
        shell_sort(&mut arr);
        assert_eq!(arr, vec!["a", "b", "c", "d"]);
    }
}
