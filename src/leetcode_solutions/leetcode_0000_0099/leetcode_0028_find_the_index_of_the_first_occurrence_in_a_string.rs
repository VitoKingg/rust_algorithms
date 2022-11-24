struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        println!("{}", &haystack[0..needle.len()]);

        match haystack.find(&needle) {
            Some(i) => i as i32,
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0028() {
        let haystack = String::from("sadbutsad");
        let needle = String::from("sad");
        let result = 0;
        assert_eq!(Solution::str_str(haystack, needle), result);

        let haystack = String::from("leetcode");
        let needle = String::from("leeto");
        let result = -1;
        assert_eq!(Solution::str_str(haystack, needle), result);
    }
}
