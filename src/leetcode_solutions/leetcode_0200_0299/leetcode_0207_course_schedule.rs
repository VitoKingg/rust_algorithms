#![allow(dead_code)]
#![allow(unused_variables)]

struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0207() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        assert!(Solution::can_finish(num_courses, prerequisites));

        let num_courses = 2;
        let prerequisites = vec![vec![1, 0], vec![0, 1]];
        assert!(!Solution::can_finish(num_courses, prerequisites));
    }
}
