pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }

    let mut sorted = false;
    let mut n = arr.len();

    while !sorted {
        sorted = true;

        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }

        n -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn bubble_sort_test() {
        // unsorted
        let mut v1 = vec![5, 7, 2, 3, 1];
        bubble_sort(&mut v1);
        assert_eq!(v1, vec![1, 2, 3, 5, 7]);

        // descending
        let mut v2 = vec![5, 4, 3, 2, 1];
        bubble_sort(&mut v2);
        assert_eq!(v2, vec![1, 2, 3, 4, 5]);

        // accending
        let mut v3 = vec![1, 2, 3, 4, 5];
        bubble_sort(&mut v3);
        assert_eq!(v3, vec![1, 2, 3, 4, 5]);

        // empty
        let mut v4: Vec<usize> = vec![];
        bubble_sort(&mut v4);
        assert_eq!(v4, vec![]);
    }
}
