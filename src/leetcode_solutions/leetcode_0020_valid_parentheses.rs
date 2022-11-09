use super::Solution;

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
                    None => {
                        return false;
                    }
                },
                _ => {}
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod leetcode_tests {
    use super::Solution;

    #[test]
    fn leetcode_0020_test() {
        assert!(Solution::is_valid(String::from("()")));
        assert!(Solution::is_valid(String::from("()[]{}")));
        assert!(!Solution::is_valid(String::from("(]")));

        assert!(Solution::is_valid(String::from("(2+3){func}[abc]")));
        assert!(!Solution::is_valid(String::from("(2+3)*(3-1")));
    }
}
