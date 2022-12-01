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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        Self::dfs(&root, &mut result);

        result
    }

    fn dfs(node_opt: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node_ptr) = node_opt {
            let node = node_ptr.borrow();
            result.push(node.val);
            Self::dfs(&node.left, result);
            Self::dfs(&node.right, result);
        }
    }

    /// iterative solution using stack
    pub fn preorder_traversal_v2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        let mut result: Vec<i32> = vec![];
        stack.push(root);

        while let Some(node_opt) = stack.pop() {
            if let Some(node_ptr) = node_opt {
                let node = node_ptr.borrow();
                result.push(node.val);
                stack.push(node.right.clone());
                stack.push(node.left.clone());
            }
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
        let result = vec![1, 2, 3];
        assert_eq!(Solution::preorder_traversal(root), result);

        let root: Option<Rc<RefCell<TreeNode>>> = None;
        let result = vec![];
        assert_eq!(Solution::preorder_traversal(root), result);

        let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let result = vec![1];
        assert_eq!(Solution::preorder_traversal(root), result);
    }
}
