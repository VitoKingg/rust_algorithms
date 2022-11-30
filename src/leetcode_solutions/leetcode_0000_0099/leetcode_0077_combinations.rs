struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut curr: Vec<i32> = vec![];

        Self::dfs(1, n, k, &mut result, &mut curr);

        result
    }

    fn dfs(start: i32, n: i32, k: i32, result: &mut Vec<Vec<i32>>, curr: &mut Vec<i32>) {
        if k == 0 {
            result.push(curr.to_vec());
            return;
        }

        for i in start..=n {
            curr.push(i);
            Self::dfs(i + 1, n, k - 1, result, curr);
            curr.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0077() {
        let n = 4;
        let k = 2;
        let result = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ];
        assert_eq!(Solution::combine(n, k), result);

        let n = 1;
        let k = 1;
        let result = vec![vec![1]];
        assert_eq!(Solution::combine(n, k), result);
    }
}
