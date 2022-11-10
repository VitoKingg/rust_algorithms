pub fn sum_recursively(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        nums[0]
    } else {
        nums[0] + sum_recursively(&nums[1..])
    }
}

#[cfg(test)]
mod math_tests {
    use super::sum_recursively;

    #[test]
    fn test_sum_recursively() {
        let nums = [1, 3, 9, 2, 4];
        assert_eq!(sum_recursively(&nums), 19);
    }
}
