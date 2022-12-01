struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as u32;
        let mut left = 1;
        let mut right = x + 1;

        while left < right {
            let mid = (left + right) / 2;

            if mid > x / mid {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        (left - 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0069() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(2147483647), 46340);
    }
}
