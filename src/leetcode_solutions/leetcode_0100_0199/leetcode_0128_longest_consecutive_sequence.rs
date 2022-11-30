use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;

        if n < 2 {
            return n;
        }

        let hs: HashSet<i32> = HashSet::from_iter(nums);
        let mut result = 1;

        for &item in &hs {
            if hs.contains(&(item - 1)) {
                continue;
            }

            let mut temp = 1;
            while hs.contains(&(item + temp)) {
                temp += 1;
                result = result.max(temp);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0128() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(Solution::longest_consecutive(nums), 4);

        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(Solution::longest_consecutive(nums), 9);

        let nums = vec![0];
        assert_eq!(Solution::longest_consecutive(nums), 1);

        let nums = vec![];
        assert_eq!(Solution::longest_consecutive(nums), 0);
    }
}
