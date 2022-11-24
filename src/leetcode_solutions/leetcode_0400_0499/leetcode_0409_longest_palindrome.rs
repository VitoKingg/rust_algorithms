use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn longest_palindrom(s: String) -> i32 {
        let mut hs: HashSet<char> = HashSet::new();
        let mut result = 0;

        for ch in s.chars() {
            if hs.contains(&ch) {
                hs.remove(&ch);
                result += 1;
            } else {
                hs.insert(ch);
            }
        }

        if hs.is_empty() {
            result * 2
        } else {
            result * 2 + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0049() {
        let s = String::from("abccccdd");
        assert_eq!(Solution::longest_palindrom(s), 7);

        let s = String::from("a");
        assert_eq!(Solution::longest_palindrom(s), 1);

        let s = String::from("aa");
        assert_eq!(Solution::longest_palindrom(s), 2);

        let s = String::from("aA");
        assert_eq!(Solution::longest_palindrom(s), 1);

        let s = String::from("aab");
        assert_eq!(Solution::longest_palindrom(s), 3);

        let s = String::from("ccc");
        assert_eq!(Solution::longest_palindrom(s), 3);
    }
}
