use std::collections::HashMap;
use std::collections::HashSet;

struct Solution;

impl Solution {
    // T(n) = O(n)
    // S(n) = O(n)
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hm: HashMap<char, usize> = HashMap::new();
        let mut result: usize = 0;
        let mut left: usize = 0;

        for (right, ch) in s.char_indices() {
            if let Some(end) = hm.insert(ch, right) {
                left = left.max(end + 1);
            }

            result = result.max(right - left + 1);
        }

        result as i32
    }

    // T(n) = O(n ^ 2)
    // S(n) = O(n)
    pub fn length_of_longest_substring_v2(s: String) -> i32 {
        let mut hs: HashSet<char> = HashSet::new();
        let s: Vec<char> = s.chars().collect();
        let mut result: usize = 0;

        let mut i = 0;
        while i < s.len() {
            hs.insert(s[i]);
            let mut j = i + 1;
            while j < s.len() {
                if hs.contains(&s[j]) {
                    break;
                } else {
                    hs.insert(s[j]);
                    j += 1;
                }
            }
            result = result.max(hs.len());
            hs.clear();

            i += 1;
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0003() {
        let s = String::from("abcabcbb");
        let result = 3;
        assert_eq!(Solution::length_of_longest_substring(s), result);

        let s = String::from("bbbbb");
        let result = 1;
        assert_eq!(Solution::length_of_longest_substring(s), result);

        let s = String::from("pwwkew");
        let result = 3;
        assert_eq!(Solution::length_of_longest_substring(s), result);
    }
}
