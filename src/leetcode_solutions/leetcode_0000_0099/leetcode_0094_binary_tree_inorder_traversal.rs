use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

struct Solution;

impl Solution {
    /// recursive solution using dfs
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        Self::dfs(&root, &mut result);

        result
    }

    fn dfs(node_opt: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node_ptr) = node_opt {
            let node = node_ptr.borrow();
            Self::dfs(&node.left, result);
            result.push(node.val);
            Self::dfs(&node.right, result);
        }
    }

    /// iterative solution using stack
    pub fn inorder_traversal_v2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        let mut result: Vec<i32> = vec![];
        let mut node_opt = root;

        while node_opt.is_some() || !stack.is_empty() {
            while let Some(node_ptr) = node_opt {
                let left_node_opt = node_ptr.borrow_mut().left.take();
                stack.push(Some(node_ptr));
                node_opt = left_node_opt;
            }

            let node_ptr = stack.pop().unwrap().unwrap();
            result.push(node_ptr.borrow().val);
            node_opt = node_ptr.borrow_mut().right.take();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0144() {
        let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        let result = vec![1, 3, 2];
        assert_eq!(Solution::inorder_traversal(root), result);

        let root: Option<Rc<RefCell<TreeNode>>> = None;
        let result = vec![];
        assert_eq!(Solution::inorder_traversal(root), result);

        let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let result = vec![1];
        assert_eq!(Solution::inorder_traversal(root), result);
    }

    #[test]
    fn test_leetcode_0144_v2() {
        let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        let result = vec![1, 3, 2];
        assert_eq!(Solution::inorder_traversal_v2(root), result);

        let root: Option<Rc<RefCell<TreeNode>>> = None;
        let result = vec![];
        assert_eq!(Solution::inorder_traversal_v2(root), result);

        let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let result = vec![1];
        assert_eq!(Solution::inorder_traversal_v2(root), result);
    }
}
