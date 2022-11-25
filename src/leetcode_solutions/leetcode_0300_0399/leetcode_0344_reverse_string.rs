struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
    }

    pub fn reverse_string_v2(s: &mut [char]) {
        s.reverse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0344() {
        let mut s: Vec<char> = "hello".chars().collect();
        let result: Vec<char> = "olleh".chars().collect();
        Solution::reverse_string(&mut s);
        assert_eq!(s, result);

        let mut s: Vec<char> = "Hannah".chars().collect();
        let result: Vec<char> = "hannaH".chars().collect();
        Solution::reverse_string(&mut s);
        assert_eq!(s, result);
    }
}
