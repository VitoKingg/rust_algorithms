#![allow(clippy::needless_range_loop)]

struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let s1: Vec<char> = text1.chars().collect();
        let s2: Vec<char> = text2.chars().collect();
        let m = s1.len();
        let n = s2.len();
        let mut dp: Vec<Vec<usize>> = vec![vec![0; n + 1]; m + 1];

        for i in 0..m {
            for j in 0..n {
                if s1[i] == s2[j] {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
                }
            }
        }

        dp[m][n] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_1143() {
        let text1 = String::from("abcde");
        let text2 = String::from("ace");
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 3);

        let text1 = String::from("abc");
        let text2 = String::from("abc");
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 3);

        let text1 = String::from("abc");
        let text2 = String::from("def");
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 0);
    }
}
