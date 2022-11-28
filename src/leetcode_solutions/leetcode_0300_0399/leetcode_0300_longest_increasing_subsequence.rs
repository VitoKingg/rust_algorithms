struct Solution;

impl Solution {
    /// the Rust way: use slice::binary_search()
    ///
    /// If the value is found then `Result::Ok` is returned, containing the index of the matching element.
    /// If there are multiple matches, then any one of the matches could be returned.
    /// The index is chosen deterministically, but is subject to change in future versions of Rust.
    /// If the value is not found then `Result::Err` is returned,
    ///     containing the index where a matching element could be inserted while maintaining sorted order.
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![];

        for num in nums {
            if let Err(i) = dp.binary_search(&num) {
                if i == dp.len() {
                    dp.push(num);
                } else {
                    dp[i] = num;
                }
            }
        }

        dp.len() as i32
    }

    pub fn length_of_lis_v2(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![1; n];

        for i in 1..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }

        *dp.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0300() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        assert_eq!(Solution::length_of_lis(nums), 4);

        let nums = vec![0, 1, 0, 3, 2, 3];
        assert_eq!(Solution::length_of_lis(nums), 4);

        let nums = vec![7, 7, 7, 7, 7, 7, 7];
        assert_eq!(Solution::length_of_lis(nums), 1);
    }
}
