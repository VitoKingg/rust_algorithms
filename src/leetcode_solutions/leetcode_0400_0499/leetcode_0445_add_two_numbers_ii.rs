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
        let mut p1 = l1;
        let mut p2 = l2;

        let mut stack1: Vec<i32> = vec![];
        let mut stack2: Vec<i32> = vec![];

        let mut result: Option<Box<ListNode>> = None;
        let mut carry = 0;

        while let Some(node) = p1 {
            stack1.push(node.val);
            p1 = node.next;
        }

        while let Some(node) = p2 {
            stack2.push(node.val);
            p2 = node.next;
        }

        while !stack1.is_empty() || !stack2.is_empty() || carry != 0 {
            if let Some(v) = stack1.pop() {
                carry += v;
            }

            if let Some(v) = stack2.pop() {
                carry += v;
            }

            let mut new_node = Some(Box::new(ListNode {
                val: carry % 10,
                next: None,
            }));

            if let Some(node) = result {
                new_node.as_mut().unwrap().next = Some(node);
                result = new_node;
            } else {
                result = new_node;
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
    fn test_leetcode_0445() {
        let l1 = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
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
                val: 8,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode { val: 7, next: None })),
                })),
            })),
        }));
        assert_eq!(Solution::add_two_numbers(l1, l2), result);

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
            val: 8,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 7, next: None })),
            })),
        }));
        assert_eq!(Solution::add_two_numbers(l1, l2), result);

        let l1 = None;
        let l2 = None;
        assert_eq!(Solution::add_two_numbers(l1, l2), None);
    }
}
