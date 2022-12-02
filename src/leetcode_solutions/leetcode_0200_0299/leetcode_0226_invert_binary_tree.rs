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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node_ptr) = &root {
            let mut node = node_ptr.borrow_mut();
            let left = node.left.take();
            let right = node.right.take();
            node.right = Self::invert_tree(left);
            node.left = Self::invert_tree(right);
        }

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0226() {
        let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let result: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(Solution::invert_tree(root), result);

        let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        let result: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(Solution::invert_tree(root), result);

        let root: Option<Rc<RefCell<TreeNode>>> = None;
        let result: Option<Rc<RefCell<TreeNode>>> = None;
        assert_eq!(Solution::invert_tree(root), result);
    }
}
