// lower Rust version needs this declaration
use std::iter::FromIterator;

struct Solution;

impl Solution {
    /// enum solution: 枚举回文中间点
    /// T(n) = O(n ^ 2)
    /// S(n) = O(n)
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut end = 0;

        for i in 0..n {
            let mut left = i;
            let mut right = i;

            while right + 1 < n && s[left] == s[right + 1] {
                right += 1;
            }

            while left > 0 && right + 1 < n && s[left - 1] == s[right + 1] {
                left -= 1;
                right += 1;
            }

            if right - left > end - start {
                start = left;
                end = right;
            }
        }

        String::from_iter(&s[start..=end])
    }

    // todo: dp solution
    /// T(n) = O(n ^ 2)
    /// S(n) = O(n ^ 2)
    pub fn longest_palindrome_v2(s: String) -> String {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0005() {
        let s = String::from("babad");
        let result = String::from("bab");
        assert_eq!(Solution::longest_palindrome(s), result);

        let s = String::from("cbbd");
        let result = String::from("bb");
        assert_eq!(Solution::longest_palindrome(s), result);

        let s = String::from("cbbbd");
        let result = String::from("bbb");
        assert_eq!(Solution::longest_palindrome(s), result);
    }
}
