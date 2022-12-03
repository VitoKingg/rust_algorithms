/// [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance)
struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut result = 0;
        let mut z = x ^ y;

        while z != 0 {
            result += z & 1;
            z >>= 1;
        }

        result
    }

    pub fn hamming_distance2(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_leetcode_0461() {
        assert_eq!(Solution::hamming_distance(1, 2), 2);
        assert_eq!(Solution::hamming_distance2(1, 2), 2);

        assert_eq!(Solution::hamming_distance(3, 1), 1);
        assert_eq!(Solution::hamming_distance2(3, 1), 1);
    }
}
