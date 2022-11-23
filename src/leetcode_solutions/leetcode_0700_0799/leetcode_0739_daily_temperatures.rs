struct Solution;

impl Solution {
    /// T(n) = O(n)
    /// S(n) = O(n)
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut stack: Vec<usize> = vec![];
        let mut result: Vec<i32> = vec![0; n];

        for i in 0..n {
            while !stack.is_empty() && temperatures[*stack.last().unwrap()] < temperatures[i] {
                let j = stack.pop().unwrap();
                result[j] = (i - j) as i32;
            }

            stack.push(i);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0739() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let result = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(Solution::daily_temperatures(temperatures), result);

        let temperatures = vec![30, 40, 50, 60];
        let result = vec![1, 1, 1, 0];
        assert_eq!(Solution::daily_temperatures(temperatures), result);

        let temperatures = vec![30, 60, 90];
        let result = vec![1, 1, 0];
        assert_eq!(Solution::daily_temperatures(temperatures), result);

        let temperatures = vec![55, 38, 53, 81, 61, 93, 97, 32, 43, 78];
        let result = vec![3, 1, 1, 2, 1, 1, 0, 1, 1, 0];
        assert_eq!(Solution::daily_temperatures(temperatures), result);
    }
}
