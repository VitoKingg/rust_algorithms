struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = vec![];
        let s: Vec<char> = s.chars().collect();
        let mut indices: Vec<(usize, usize)> = vec![];

        Self::dfs(&s, &mut indices, &mut result, 0);

        result
    }

    fn dfs(
        s: &[char],
        indices: &mut Vec<(usize, usize)>,
        result: &mut Vec<Vec<String>>,
        start: usize,
    ) {
        let n = s.len();

        if start == n {
            let mut parts: Vec<String> = vec![];

            for &(l, r) in indices.iter() {
                parts.push(s[l..r].iter().collect());
            }

            result.push(parts);
        }

        for end in (start + 1)..=n {
            if Self::is_palindrome(&s[start..end]) {
                indices.push((start, end));
                Self::dfs(s, indices, result, end);
                indices.pop();
            }
        }
    }

    fn is_palindrome(s: &[char]) -> bool {
        let a: String = s.iter().collect();
        let b: String = s.iter().rev().collect();

        a == b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0131() {
        let s = String::from("aab");
        let result = vec![
            vec![String::from("a"), String::from("a"), String::from("b")],
            vec![String::from("aa"), String::from("b")],
        ];
        assert_eq!(Solution::partition(s), result);

        let s = String::from("a");
        let result = vec![vec![String::from("a")]];
        assert_eq!(Solution::partition(s), result);

        let s = String::from("abcba");
        let result = vec![
            vec![
                String::from("a"),
                String::from("b"),
                String::from("c"),
                String::from("b"),
                String::from("a"),
            ],
            vec![String::from("a"), String::from("bcb"), String::from("a")],
            vec![String::from("abcba")],
        ];
        assert_eq!(Solution::partition(s), result);
    }
}
