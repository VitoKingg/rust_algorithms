struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut nums = nums;

        Self::dfs(&mut nums, &mut result, 0);

        result
    }

    fn dfs(nums: &mut [i32], result: &mut Vec<Vec<i32>>, start: usize) {
        let n = nums.len();

        if start == n {
            result.push(nums.to_vec());
            return;
        }

        for i in start..n {
            nums.swap(start, i);
            Self::dfs(nums, result, start + 1);
            nums.swap(start, i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0046() {
        let nums = vec![1, 2, 3];
        let result = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 2, 1],
            vec![3, 1, 2],
        ];
        assert_eq!(Solution::permute(nums), result);

        let nums = vec![0, 1];
        let result = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(Solution::permute(nums), result);

        let nums = vec![1];
        let result = vec![vec![1]];
        assert_eq!(Solution::permute(nums), result);
    }
}
