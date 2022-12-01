use std::convert::TryInto;

struct Solution;

impl Solution {
    /// T(n) = O(n)
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n: usize = n.try_into().unwrap();
        let mut result: Vec<i32> = vec![0; n + 1];

        for i in 1..=n {
            result[i] = result[i & (i - 1)] + 1;
        }

        result
    }

    /// T(n) = O()
    pub fn count_bits_v2(n: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        for i in 0..=n {
            result.push(i.count_ones() as i32);
        }

        result
    }

    /// T(n) = O(n * log(n))
    pub fn count_bits_v3(n: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        for i in 0..=n {
            let mut sum = 0;
            let mut num = i;

            while num != 0 {
                sum += num % 2;
                num /= 2;
            }

            result.push(sum);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0338() {
        let n = 2;
        let result = vec![0, 1, 1];
        assert_eq!(Solution::count_bits(n), result);
        assert_eq!(Solution::count_bits_v2(n), result);
        assert_eq!(Solution::count_bits_v3(n), result);

        let n = 5;
        let result = vec![0, 1, 1, 2, 1, 2];
        assert_eq!(Solution::count_bits(n), result);
        assert_eq!(Solution::count_bits_v2(n), result);
        assert_eq!(Solution::count_bits_v3(n), result);
    }
}
