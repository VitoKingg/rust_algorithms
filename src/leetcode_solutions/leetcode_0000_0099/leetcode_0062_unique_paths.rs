struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];

        for i in 1..=m {
            for j in 1..=n {
                if i == 1 && j == 1 {
                    dp[i][j] = 1;
                } else {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                }
            }
        }

        dp[m][n]
    }

    pub fn unique_paths_v2(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp: Vec<i32> = vec![1; n];

        for _i in 1..m {
            for j in 1..n {
                dp[j] += dp[j - 1];
            }
        }

        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0062() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }
}
