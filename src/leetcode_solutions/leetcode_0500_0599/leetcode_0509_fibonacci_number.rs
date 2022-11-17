/// [LeetCode 0509 Fibonacci Number](https://leetcode.com/problems/fibonacci-number/)
/// [Fibonacci number](https://en.wikipedia.org/wiki/Fibonacci_number)
struct Solution;

impl Solution {
    /// fib(n) returns the nth fibonacci number
    /// F(0) = 0, F(1) = 1 and F(n) = F(n-1) + F(n-2) for n > 1
    /// 动态规划迭代版: T(n) = O(n), S(n) = O(1)
    ///
    /// Warning: This function will overflow the 32-bit integer at n = 46
    pub fn fib(n: i32) -> i32 {
        let mut a = 0;
        let mut b = 1;

        for _ in 0..n {
            b += a;
            a = b - a;
        }

        a
    }

    /// fibonacci(n) returns the nth fibonacci number
    /// F(0) = 0, F(1) = 1 and F(n) = F(n-1) + F(n-2) for n > 1
    /// 动态规划迭代版: T(n) = O(n), S(n) = O(1)
    ///
    /// Warning: This function will overflow the 128-bit unsigned integer at n = 186
    pub fn fibonacci(n: u32) -> u128 {
        let mut a = 0;
        let mut b = 1;

        for _ in 0..n {
            b += a;
            a = b - a;
        }

        a
    }

    /// fib(n) returns the nth fibonacci number
    /// F(0) = 0, F(1) = 1 and F(n) = F(n-1) + F(n-2) for n > 1
    /// memorized fibonacci: T(n) = O(n), S(n) = O(n)
    ///
    /// Warning: This function will overflow the 32-bit integer at n = 187
    pub fn fibonacci_memorized(n: u32) -> u128 {
        let n = n as usize;
        let mut a: Vec<u128> = vec![0; n + 1];

        a[1] = 1;
        for i in 2..=n {
            a[i] = a[i - 1] + a[i - 2];
        }

        a[n]
    }

    /// fib(n) returns the nth fibonacci number
    /// F(0) = 0, F(1) = 1 and F(n) = F(n-1) + F(n-2) for n > 1
    /// 动态规划迭代版: T(n) = O(n), S(n) = O(1)
    ///
    /// Warning: This function will overflow the 128-bit unsigned integer at n = 186
    pub fn fibonacci_recursion(n: u32) -> u128 {
        Self::_fibonacci_recursion(n, 0, 1)
    }

    fn _fibonacci_recursion(n: u32, curr: u128, next: u128) -> u128 {
        if n == 0 {
            curr
        } else {
            Self::_fibonacci_recursion(n - 1, next, curr + next)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_leetcode_0509() {
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fibonacci(2), 1);
        assert_eq!(Solution::fibonacci_memorized(2), 1);
        assert_eq!(Solution::fibonacci_recursion(2), 1);

        assert_eq!(Solution::fib(5), 5);
        assert_eq!(Solution::fibonacci(5), 5);
        assert_eq!(Solution::fibonacci_memorized(5), 5);
        assert_eq!(Solution::fibonacci_recursion(5), 5);

        assert_eq!(Solution::fib(10), 55);
        assert_eq!(Solution::fibonacci(10), 55);
        assert_eq!(Solution::fibonacci_memorized(10), 55);
        assert_eq!(Solution::fibonacci_recursion(10), 55);

        use std::time::Instant;
        let now = Instant::now();
        Solution::fibonacci(185);
        println!("{}", now.elapsed().as_micros());

        let now = Instant::now();
        Solution::fibonacci_recursion(185);
        println!("{}", now.elapsed().as_micros());

        let now = Instant::now();
        Solution::fibonacci_memorized(185);
        println!("{}", now.elapsed().as_micros());
    }
}
