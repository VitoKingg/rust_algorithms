/// [LeetCode 0125 Valid Palindrome](https://leetcode.com/problems/valid-palindrome/)
struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        let a: String = s.iter().collect();
        let b: String = s.iter().rev().collect();

        a == b
    }
}

#[cfg(test)]
mod leetcode_tests {
    use super::Solution;

    #[test]
    fn test_leetcode_0125() {
        assert!(Solution::is_palindrome(String::from("121")));
        assert!(Solution::is_palindrome(String::from("12321")));
        assert!(!Solution::is_palindrome(String::from("1234")));
        assert!(Solution::is_palindrome(String::from(
            "A man, a plan, a canal: Panama"
        )));
        assert!(!Solution::is_palindrome(String::from("race a car")));
        assert!(Solution::is_palindrome(String::from(" ")));
    }
}
