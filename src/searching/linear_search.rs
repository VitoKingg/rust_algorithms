/// Rust std::*::contains()
/// T(n) = O(n/2) = O(n)
pub fn linear_search<T: PartialEq>(arr: &[T], item: &T) -> Option<usize> {
    for (i, data) in arr.iter().enumerate() {
        if data == item {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        let arr = vec![1, 2, 3, 4, 5, 6];
        let item = 3;
        assert_eq!(linear_search(&arr, &item), Some(2));

        let arr = [1, 2, 3, 4, 5, 6];
        let item = 10;
        assert_eq!(linear_search(&arr, &item), None);

        let arr = ["abc", "abcd", "abd", "google", "mozilla", "microsoft"];
        let item = "abd";
        assert_eq!(linear_search(&arr, &item), Some(2));

        let arr = vec![];
        let item = 10;
        assert_eq!(linear_search(&arr, &item), None);
    }
}
