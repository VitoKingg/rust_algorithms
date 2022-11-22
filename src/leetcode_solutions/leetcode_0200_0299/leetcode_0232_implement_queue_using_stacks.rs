struct MyQueue {
    stack: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        self.stack.insert(0, x);
    }

    fn pop(&mut self) -> i32 {
        if let Some(x) = self.stack.pop() {
            return x;
        }

        0
    }

    fn peek(&self) -> i32 {
        self.stack[self.stack.len() - 1]
    }

    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::MyQueue;

    #[test]
    fn test_leetcode_0232() {
        let mut obj = MyQueue::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.peek(), 1);
        assert_eq!(obj.pop(), 1);
        assert!(!obj.empty());
        assert_eq!(obj.pop(), 2);
        assert!(obj.empty());
    }
}
