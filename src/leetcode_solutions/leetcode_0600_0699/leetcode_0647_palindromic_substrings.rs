struct Solution;

impl Solution {
    /// Manacher algorithm
    /// T(n) = O(n)
    /// S(n) = O(n)
    pub fn count_substrings(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len() as i32;
        let mut result = 0;

        let mut k: i32 = 0;
        while k < n * 2 - 1 {
            let mut i: i32 = k / 2;
            let mut j: i32 = (k + 1) / 2;

            while i >= 0 && j < n && s[i as usize] == s[j as usize] {
                result += 1;
                i -= 1;
                j += 1;
            }

            k += 1;
        }

        result
    }

    pub fn count_substrings_v2(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut result: i32 = 0;
        let mut dp = vec![vec![false; n + 1]; n + 1];

        for i in (0..n).rev() {
            for j in i..n {
                if (j == i)
                    || (j == i + 1 && s[i] == s[j])
                    || (j > i + 1 && s[i] == s[j] && dp[i + 1][j - 1])
                {
                    dp[i][j] = true;
                    result += 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0647() {
        let s = String::from("abc");
        assert_eq!(Solution::count_substrings(s), 3);

        let s = String::from("aaa");
        assert_eq!(Solution::count_substrings(s), 6);

        let s = String::from("abcba");
        assert_eq!(Solution::count_substrings(s), 7);
    }
}
