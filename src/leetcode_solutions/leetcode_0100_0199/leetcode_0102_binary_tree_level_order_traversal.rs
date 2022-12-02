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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];

        Self::bfs(&root, &mut result, 0);

        result
    }

    fn bfs(node_opt: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<Vec<i32>>, level: usize) {
        if let Some(node_ptr) = node_opt {
            let node = node_ptr.borrow();
            let node_val = node.val;

            if result.len() == level {
                result.push(vec![node_val]);
            } else {
                result[level].push(node_val);
            }

            Self::bfs(&node.left, result, level + 1);
            Self::bfs(&node.right, result, level + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0144() {
        let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let result = vec![vec![3], vec![9, 20], vec![15, 7]];
        assert_eq!(Solution::level_order(root), result);

        let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let result = vec![vec![1], vec![2, 3], vec![4, 5]];
        assert_eq!(Solution::level_order(root), result);

        let root: Option<Rc<RefCell<TreeNode>>> = None;
        let result: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::level_order(root), result);

        let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let result = vec![vec![1]];
        assert_eq!(Solution::level_order(root), result);
    }
}
