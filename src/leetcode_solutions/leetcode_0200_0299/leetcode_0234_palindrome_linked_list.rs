struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

struct Solution;

impl Solution {
    // todo: need some optimization
    pub fn is_palindrom(head: Option<Box<ListNode>>) -> bool {
        let mut head_copy = head;
        let mut list: Vec<i32> = vec![];

        while let Some(ref node) = head_copy {
            list.push(node.val);
            head_copy = head_copy.unwrap().next;
        }

        // for (i, &v) in list.iter().rev().enumerate() {
        //     if list[i] != v {
        //         return false;
        //     }
        // }

        for i in 0..list.len() / 2 {
            if list[i] != list[list.len() - i - 1] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0234() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 1, next: None })),
                })),
            })),
        }));
        assert!(Solution::is_palindrom(head));

        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));
        assert!(!Solution::is_palindrom(head));
    }
}
