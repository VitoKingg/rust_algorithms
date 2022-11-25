struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut left: i32 = 1;
        let mut right: i32 = (nums.len() - 1) as i32;

        while left < right {
            let mid = left + (right - left) / 2;
            let mut count = 0;

            for &num in &nums {
                if num <= mid {
                    count += 1;
                }
            }

            if count <= mid {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0287() {
        let nums = vec![1, 3, 4, 2, 2];
        let result = 2;
        assert_eq!(Solution::find_duplicate(nums), result);

        let nums = vec![3, 1, 3, 4, 2];
        let result = 3;
        assert_eq!(Solution::find_duplicate(nums), result);
    }
}
