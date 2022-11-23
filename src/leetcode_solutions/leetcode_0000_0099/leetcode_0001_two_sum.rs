use std::collections::HashMap;

struct Solution;

impl Solution {
    /// T(n) = O(n)
    /// S(n) = O(n)
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, v) in nums.iter().enumerate() {
            if map.contains_key(v) {
                return vec![map[v], i as i32];
            } else {
                map.insert(target - nums[i], i as i32);
            }
        }

        vec![]
    }

    /// T(n) = O(n^2)
    /// S(n) = O(1)
    pub fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0, 0];

        for i in 0..n {
            for j in i + 1..n {
                if (nums[i] + nums[j]) == target {
                    result[0] = i as i32;
                    result[1] = j as i32;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0001() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, target), result);

        let nums = vec![3, 2, 4];
        let target = 6;
        let result = vec![1, 2];
        assert_eq!(Solution::two_sum(nums, target), result);

        let nums = vec![3, 3];
        let target = 6;
        let result = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, target), result);
    }
}
