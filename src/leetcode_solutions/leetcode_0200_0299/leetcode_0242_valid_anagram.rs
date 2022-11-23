use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s: Vec<char> = s.chars().collect();
        let mut t: Vec<char> = t.chars().collect();

        s.sort_unstable();
        t.sort_unstable();
        s == t
    }

    pub fn is_anagram_v2(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut record: HashMap<char, i32> = HashMap::new();
        for ch in s.chars() {
            record.entry(ch).and_modify(|v| *v += 1).or_insert(1);
        }

        for ch in t.chars() {
            match record.get_mut(&ch) {
                Some(v) => {
                    if *v > 0 {
                        *v -= 1;
                    } else {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_anagram_v3(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let (s, t) = (s.as_bytes(), t.as_bytes());
        let mut record = vec![0; 26];
        for i in 0..s.len() {
            record[(s[i] - b'a') as usize] += 1;
            record[(t[i] - b'a') as usize] -= 1;
        }

        record.iter().all(|&c| c == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0242() {
        let s = String::from("anagram");
        let t = String::from("nagaram");
        assert!(Solution::is_anagram(s, t));

        let s = String::from("rat");
        let t = String::from("car");
        assert!(!Solution::is_anagram(s, t));
    }
}
