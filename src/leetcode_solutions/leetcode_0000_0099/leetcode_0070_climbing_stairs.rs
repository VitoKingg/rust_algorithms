struct Solution;

impl Solution {
    /// Fibonacci
    pub fn climb_stairs(n: i32) -> i32 {
        let mut a = 1;
        let mut b = 1;
        let mut temp;

        for _i in 1..n {
            temp = a + b;
            a = b;
            b = temp;
        }

        b
    }

    /// Fibonacci
    /// use Rust iterator
    pub fn climb_stairs_v2(n: i32) -> i32 {
        match n {
            1 | 2 => n,
            k => (2..k).fold((1, 2), |acc, _| (acc.1, acc.0 + acc.1)).1,
        }
    }

    /// recursion solution: time limit exceeded
    pub fn climb_stairs_v3(n: i32) -> i32 {
        match n {
            1 | 2 => n,
            _ => Self::climb_stairs(n - 1) + Self::climb_stairs(n - 2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0070() {
        let n = 2;
        let result = 2;
        assert_eq!(Solution::climb_stairs(n), result);
        assert_eq!(Solution::climb_stairs_v2(n), result);

        let n = 3;
        let result = 3;
        assert_eq!(Solution::climb_stairs(n), result);
        assert_eq!(Solution::climb_stairs_v2(n), result);

        let n = 4;
        let result = 5;
        assert_eq!(Solution::climb_stairs(n), result);
        assert_eq!(Solution::climb_stairs_v2(n), result);

        let n = 5;
        let result = 8;
        assert_eq!(Solution::climb_stairs(n), result);
        assert_eq!(Solution::climb_stairs_v2(n), result);
    }
}
