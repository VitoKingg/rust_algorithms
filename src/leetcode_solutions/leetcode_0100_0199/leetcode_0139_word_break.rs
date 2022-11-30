use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        let hs: HashSet<String> = HashSet::from_iter(word_dict);
        let mut dp = vec![false; n + 1];
        dp[0] = true;

        for i in 1..=n {
            for j in 0..i {
                if dp[j] && hs.contains(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0139() {
        let s = String::from("leetcode");
        let word_dict = vec![String::from("leet"), String::from("code")];
        assert!(Solution::word_break(s, word_dict));

        let s = String::from("applepenapple");
        let word_dict = vec![String::from("apple"), String::from("pen")];
        assert!(Solution::word_break(s, word_dict));

        let s = String::from("catsandog");
        let word_dict = vec![
            String::from("cats"),
            String::from("dog"),
            String::from("sand"),
            String::from("and"),
            String::from("cat"),
        ];
        assert!(!Solution::word_break(s, word_dict));
    }
}
