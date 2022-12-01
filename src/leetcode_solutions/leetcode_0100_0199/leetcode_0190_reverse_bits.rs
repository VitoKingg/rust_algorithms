struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut x = x;
        let mut result = 0;

        for _ in 0..32 {
            result = (result << 1) | (x & 1);
            x >>= 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0190() {
        let n = 0b00000010100101000001111010011100;
        let result = 964176192;
        assert_eq!(Solution::reverse_bits(n), result);

        let n = 0b11111111111111111111111111111101;
        let result = 3221225471;
        assert_eq!(Solution::reverse_bits(n), result);
    }
}
