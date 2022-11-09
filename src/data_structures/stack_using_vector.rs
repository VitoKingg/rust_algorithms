#[derive(Debug)]
pub struct StackV<T> {
    top: usize,
    data: Vec<T>,
}

impl<T> StackV<T> {
    pub fn new() -> Self {
        Self {
            top: 0,
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }

        self.top -= 1;
        self.data.pop()
    }

    pub fn peek(&mut self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }

        self.data.get(self.top - 1)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.top == 0 {
            return None;
        }

        self.data.get_mut(self.top - 1)
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn size(&self) -> usize {
        self.top
    }
}

impl<T> Default for StackV<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod data_structures_tests {
    use super::StackV;

    #[test]
    fn stackv_test() {
        let mut s: StackV<i32> = StackV::new();
        s.push(1);
        s.push(2);
        s.push(3);
        assert!(!s.is_empty());
        assert_eq!(*s.peek().unwrap(), 3);
        *s.peek_mut().unwrap() += 2;
        assert_eq!(s.size(), 3);
        assert_eq!(s.pop().unwrap(), 5);
        assert_eq!(s.pop().unwrap(), 2);
        assert_eq!(s.pop().unwrap(), 1);
        assert_eq!(s.pop(), None);
        assert!(s.is_empty());
    }

    #[test]
    fn is_valid_parentheses_test() {
        let sa = "(2+3){func}[abc]";
        let sb = "(2+3)*(3-1";
        assert!(is_valid_parentheses(sa));
        assert!(!is_valid_parentheses(sb));
    }

    fn is_valid_parentheses(s: &str) -> bool {
        let mut stack: StackV<char> = StackV::new();

        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' | ']' | '}' => match stack.pop() {
                    Some(t) => {
                        if !((t == '(' && c == ')')
                            || (t == '[' && c == ']')
                            || (t == '{' && c == '}'))
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
