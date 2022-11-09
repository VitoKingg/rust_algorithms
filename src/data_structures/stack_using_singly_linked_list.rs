/// All Algorithms implemented in Rust
/// https://github.com/TheAlgorithms/Rust/blob/master/src/data_structures/stack_using_singly_linked_list.rs

#[derive(Debug)]
pub struct Stack<T> {
    top: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { top: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.top.take(),
        });

        self.top = Some(new_node);
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        match self.top.take() {
            Some(node) => {
                self.top = node.next;
                Ok(node.elem)
            },
            None => Err("Stack is empty"),
        }

    }

    pub fn peek(&self) -> Option<&T> {
        match self.top.as_deref() {
            Some(node) => Some(&node.elem),
            None => None,
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        match self.top.as_deref_mut() {
            Some(node) => Some(&mut node.elem),
            None => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.top.is_none()
    }

    pub fn iter() {
        // todo
    }

    pub fn iter_mut() {
        // todo
    }

    pub fn into_iter_for_stack() {
        // todo
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        // todo: implement drop function using iterator
        while !self.is_empty() {
            self.pop().unwrap();
        }
    }
}

#[cfg(test)]
mod stack_tests {
    use super::Stack;

    #[test]
    fn basics_test() {
        let mut stack: Stack<i32> = Stack::new();
        assert_eq!(stack.pop(), Err("Stack is empty"));
        for i in 0..10 {
            stack.push(i);
        }
        println!("{:?}", stack);

        while !stack.is_empty() {
            stack.pop().unwrap();
        }
        println!("{:?}", stack);
    }
}
