pub fn sum_recursion(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        nums[0]
    } else {
        nums[0] + sum_recursion(&nums[1..])
    }
}

pub fn sum_tail_recursion(sum: i32, nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        sum + nums[0]
    } else {
        sum_tail_recursion(sum + nums[0], &nums[1..])
    }
}

#[cfg(test)]
mod tests {
    use super::sum_recursion;

    #[test]
    fn test_sum_recursion() {
        let nums = [1, 3, 9, 2, 4];
        assert_eq!(sum_recursion(&nums), 19);
    }

    #[test]
    fn test_sum_tail_recursion() {
        let nums = [1, 3, 9, 2, 4];
        assert_eq!(sum_recursion(&nums), 19);
    }
}
