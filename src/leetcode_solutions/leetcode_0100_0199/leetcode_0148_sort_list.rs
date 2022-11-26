#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct Solution;

impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = head;
        let mut list: Vec<i32> = vec![];

        while let Some(node) = curr {
            list.push(node.val);
            curr = node.next;
        }

        list.sort_unstable_by(|a, b| b.cmp(a));

        let mut another_head: Option<Box<ListNode>> = None;
        let mut another_curr = &mut another_head;
        while let Some(val) = list.pop() {
            let new_node = Some(Box::new(ListNode::new(val)));

            if let Some(node) = another_curr {
                node.next = new_node;
                another_curr = &mut node.next;
            } else {
                another_head = new_node;
                another_curr = &mut another_head;
            }
        }

        another_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0148() {
        let head = Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        }));
        let result = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        }));
        assert_eq!(Solution::sort_list(head), result);

        let head = Some(Box::new(ListNode {
            val: -1,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 0, next: None })),
                    })),
                })),
            })),
        }));
        let result = Some(Box::new(ListNode {
            val: -1,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));
        assert_eq!(Solution::sort_list(head), result);

        let head: Option<Box<ListNode>> = None;
        let result: Option<Box<ListNode>> = None;
        assert_eq!(Solution::sort_list(head), result);
    }
}
