use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();
        let group: Vec<char> = s.chars().collect();

        for &ch in &group {
            map.entry(ch).and_modify(|v| *v += 1).or_insert(1);
        }

        for (i, v) in group.iter().enumerate() {
            if let Some(1) = map.get(v) {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0387() {
        let s = String::from("leetcode");
        assert_eq!(Solution::first_uniq_char(s), 0);

        let s = String::from("loveleetcode");
        assert_eq!(Solution::first_uniq_char(s), 2);

        let s = String::from("aabb");
        assert_eq!(Solution::first_uniq_char(s), -1);
    }
}
