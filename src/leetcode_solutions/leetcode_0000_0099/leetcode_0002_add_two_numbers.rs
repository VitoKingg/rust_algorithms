#[derive(PartialEq, Eq, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result: Option<Box<ListNode>> = None;
        let mut curr = &mut result;

        let mut p1 = l1;
        let mut p2 = l2;

        let mut carry = 0;
        while p1.is_some() || p2.is_some() || carry != 0 {
            if let Some(node) = p1 {
                carry += node.val;
                p1 = node.next;
            }

            if let Some(node) = p2 {
                carry += node.val;
                p2 = node.next;
            }

            let new_node = Some(Box::new(ListNode {
                val: carry % 10,
                next: None,
            }));

            if let Some(node) = curr {
                node.next = new_node;
                curr = &mut node.next;
            } else {
                result = new_node;
                curr = &mut result;
            }

            carry /= 10;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0002() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let result = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            })),
        }));
        assert_eq!(Solution::add_two_numbers(l1, l2), result);

        let l1 = Some(Box::new(ListNode { val: 0, next: None }));
        let l2 = Some(Box::new(ListNode { val: 0, next: None }));
        let result = Some(Box::new(ListNode { val: 0, next: None }));
        assert_eq!(Solution::add_two_numbers(l1, l2), result);

        let l1 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode { val: 9, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode { val: 9, next: None })),
                })),
            })),
        }));
        let result = Some(Box::new(ListNode {
            val: 8,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 0,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: 0,
                                    next: Some(Box::new(ListNode { val: 1, next: None })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(Solution::add_two_numbers(l1, l2), result);
    }
}
