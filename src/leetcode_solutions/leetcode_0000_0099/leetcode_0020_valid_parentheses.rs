struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];

        for ch in s.chars() {
            match ch {
                '(' | '[' | '{' => stack.push(ch),
                ')' | ']' | '}' => match stack.pop() {
                    Some(t) => {
                        if !((t == '(' && ch == ')')
                            || (t == '[' && ch == ']')
                            || (t == '{' && ch == '}'))
                        {
                            return false;
                        }
                    }
                    None => return false,
                },
                _ => {}
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_leetcode_0020() {
        assert!(Solution::is_valid(String::from("()")));
        assert!(Solution::is_valid(String::from("()[]{}")));
        assert!(!Solution::is_valid(String::from("(]")));

        assert!(Solution::is_valid(String::from("(2+3){func}[abc]")));
        assert!(!Solution::is_valid(String::from("(2+3)*(3-1")));
    }
}
