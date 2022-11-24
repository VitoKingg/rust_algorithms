use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut hm = HashMap::new();

        for &num in &nums {
            hm.entry(num).and_modify(|v| *v += 1).or_insert(1);
        }

        for (k, v) in &hm {
            // (i + 1) and (i - 1) are both OK
            if let Some(u) = hm.get(&(k - 1)) {
                result = result.max(u + v);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0594() {
        let nums = vec![1, 3, 2, 2, 5, 2, 3, 7];
        assert_eq!(Solution::find_lhs(nums), 5);

        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::find_lhs(nums), 2);

        let nums = vec![1, 1, 1, 1];
        assert_eq!(Solution::find_lhs(nums), 0);

        let nums = vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 5, 5, 5, 5];
        assert_eq!(Solution::find_lhs(nums), 6);
    }
}
