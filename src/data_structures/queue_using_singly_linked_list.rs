use std::collections::LinkedList;

#[derive(Debug)]
pub struct Queue<T> {
    elem: LinkedList<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            elem: LinkedList::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elem.push_back(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.elem.pop_front()
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.elem.front()
    }

    pub fn is_empty(&self) -> bool {
        self.elem.is_empty()
    }

    pub fn size(&self) -> usize {
        self.elem.len()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod data_structures_tests {
    use super::Queue;

    #[test]
    fn test_enqueue() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(64);
        assert!(!queue.is_empty());
    }

    #[test]
    fn test_dequeue() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(32);
        queue.enqueue(64);
        let retrieved_dequeue = queue.dequeue();
        assert_eq!(retrieved_dequeue, Some(32));
    }

    #[test]
    fn test_peek_front() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(8);
        queue.enqueue(16);
        let retrieved_peek = queue.peek_front();
        assert_eq!(retrieved_peek, Some(&8));
    }

    #[test]
    fn test_size() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(8);
        queue.enqueue(16);
        assert_eq!(2, queue.size());
    }
}
