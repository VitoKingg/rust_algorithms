struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();

        if n < 2 {
            return 0;
        }

        let mut result = 0;
        let mut left = 0;
        let mut right = n - 1;
        let mut level = 0;

        while left < right {
            if height[left] < height[right] {
                let lower = height[left];
                level = level.max(lower);
                result += level - lower;
                left += 1;
            } else {
                let lower = height[right];
                level = level.max(lower);
                result += level - lower;
                right -= 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0042() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let result = 6;
        assert_eq!(Solution::trap(height), result);

        let height = vec![4, 2, 0, 3, 2, 5];
        let result = 9;
        assert_eq!(Solution::trap(height), result);
    }
}
