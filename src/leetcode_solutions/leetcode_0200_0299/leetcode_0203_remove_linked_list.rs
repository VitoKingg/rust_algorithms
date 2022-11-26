#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

struct Solution;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head_copy = head;
        let mut curr = &mut head_copy;

        while let Some(node) = curr {
            if node.val == val {
                *curr = curr.take().unwrap().next.take();
            } else {
                curr = &mut curr.as_mut().unwrap().next;
            }
        }

        head_copy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0203() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 5,
                                next: Some(Box::new(ListNode { val: 6, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        let val = 6;
        let result = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));
        assert_eq!(Solution::remove_elements(head, val), result);

        let head = None;
        let val = 1;
        let result = None;
        assert_eq!(Solution::remove_elements(head, val), result);

        let head = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 7,
                    next: Some(Box::new(ListNode { val: 7, next: None })),
                })),
            })),
        }));
        let val = 7;
        let result = None;
        assert_eq!(Solution::remove_elements(head, val), result);
    }
}
