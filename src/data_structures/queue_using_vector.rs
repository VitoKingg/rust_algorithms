#[derive(Debug)]
pub struct QueueV<T> {
    capacity: usize,
    elem: Vec<T>,
}

impl<T> QueueV<T> {
    pub fn new(size: usize) -> Self {
        Self {
            capacity: size,
            elem: Vec::with_capacity(size),
        }
    }

    pub fn enqueue(&mut self, val: T) -> Result<(), String> {
        if Self::size(self) == self.capacity {
            return Err(String::from("No space available"));
        }

        self.elem.insert(0, val);

        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.elem.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.elem.len() == 0
    }

    pub fn size(&self) -> usize {
        self.elem.len()
    }
}

#[cfg(test)]
mod data_structures_tests {
    use super::QueueV;

    #[test]
    fn queuev_test() {
        let mut queue: QueueV<i32> = QueueV::new(3);
        assert!(queue.is_empty());
        queue.enqueue(1).unwrap();
        queue.enqueue(2).unwrap();
        queue.enqueue(3).unwrap();
        assert!(!queue.is_empty());
        println!("{:?}", queue);
        assert_eq!(queue.dequeue().unwrap(), 1);
        assert_eq!(queue.dequeue().unwrap(), 2);
        assert_eq!(queue.dequeue().unwrap(), 3);
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.dequeue(), None);
        assert!(queue.is_empty());
    }
}
