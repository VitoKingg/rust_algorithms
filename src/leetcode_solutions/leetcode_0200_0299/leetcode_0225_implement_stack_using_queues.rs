struct MyStack {
    queue: Vec<i32>,
}

impl MyStack {
    fn new() -> Self {
        Self { queue: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        self.queue.push(x);
    }

    fn pop(&mut self) -> i32 {
        if let Some(x) = self.queue.pop() {
            return x;
        }

        0
    }

    fn top(&self) -> i32 {
        self.queue[self.queue.len() - 1]
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0225() {
        let mut obj = MyStack::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.top(), 2);
        assert_eq!(obj.pop(), 2);
        assert!(!obj.empty());
        assert_eq!(obj.pop(), 1);
        assert!(obj.empty());
    }
}
