struct Solution;

impl Solution {
    /// dp solution, space optimized, Fibonacci sequence
    /// T(n) = O(n)
    /// S(n) = O(1)
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut a = 0;
        let mut b = 0;

        for num in nums {
            let tmp = a;
            a = a.max(b + num);
            b = tmp;
        }

        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0198() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(Solution::rob(nums), 4);

        let nums = vec![2, 1, 1, 2];
        assert_eq!(Solution::rob(nums), 4);

        let nums = vec![2, 7, 9, 3, 1];
        assert_eq!(Solution::rob(nums), 12);
    }
}
