#![allow(unused_variables)]

struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0215() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let k = 2;
        let result = 5;
        assert_eq!(Solution::find_kth_largest(nums, k), result);

        let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let k = 4;
        let result = 4;
        assert_eq!(Solution::find_kth_largest(nums, k), result);
    }
}
