struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut curr: Vec<i32> = vec![];
        let mut candidates = candidates;
        candidates.sort_unstable();

        Self::dfs(0, target, &candidates, &mut result, &mut curr);

        result
    }

    fn dfs(
        start: usize,
        target: i32,
        candidates: &[i32],
        result: &mut Vec<Vec<i32>>,
        curr: &mut Vec<i32>,
    ) {
        if target == 0 {
            result.push(curr.to_vec());
            return;
        }

        let n = candidates.len();
        for i in start..n {
            if candidates[i] > target {
                break;
            } else {
                curr.push(candidates[i]);
                Self::dfs(i, target - candidates[i], candidates, result, curr);
                curr.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0039() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let result = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(Solution::combination_sum(candidates, target), result);

        let candidates = vec![2, 3, 5];
        let target = 8;
        let result = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
        assert_eq!(Solution::combination_sum(candidates, target), result);

        let candidates = vec![2];
        let target = 1;
        let result: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::combination_sum(candidates, target), result);

        let candidates = vec![8, 4, 7, 3];
        let target = 11;
        let result = vec![vec![3, 4, 4], vec![3, 8], vec![4, 7]];
        assert_eq!(Solution::combination_sum(candidates, target), result);
    }
}
