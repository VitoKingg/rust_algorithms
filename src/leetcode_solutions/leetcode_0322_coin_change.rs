/// [LeetCode 0322 Coin Change](https://leetcode.com/problems/coin-change/)
struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // make a vector of amount+1 size, count in 0.
        let n = (amount + 1) as usize;
        let mut dp = vec![-1; n];
        dp[0] = 0;

        for i in 1..=amount {
            for &c in &coins {
                if c <= i {
                    let i = i as usize;
                    let j = i - c as usize;
                    if dp[j] != -1 {
                        if dp[i] == -1 {
                            dp[i] = dp[j] + 1;
                        } else {
                            dp[i] = i32::min(dp[i], dp[j] + 1);
                        }
                    }
                }
            }
        }

        dp[amount as usize]
    }

    /// Arguments:
    ///     * `coins` - coins of different denominations
    ///     * `amount` - a total amount of money be made up
    /// Complexity:
    ///     * T(n) = O(amount * coins.length)
    ///     * S(n) = O(amount)
    /// dynamic programming, implement in Rust way.
    pub fn coin_change_rust_way(coins: &[usize], amount: usize) -> Option<usize> {
        let mut dp: Vec<Option<usize>> = vec![None; amount + 1];
        dp[0] = Some(0);

        // assume dp[i] is the fewest number of coins making up amount i
        for i in 1..=amount {
            for &c in coins {
                if c <= i {
                    dp[i] = match dp[i - c] {
                        Some(prev_coins) => match dp[i] {
                            Some(curr_coins) => Some(curr_coins.min(prev_coins + 1)),
                            None => Some(prev_coins + 1),
                        },
                        None => dp[i],
                    }
                }
            }
        }

        dp[amount]
    }
}

#[cfg(test)]
mod leetcode_tests {
    use super::Solution;

    #[test]
    fn test_leetcode_0322() {
        let coins = vec![1, 2, 5];
        let amount = 11;
        let result = 3;
        assert_eq!(Solution::coin_change(coins, amount), result);
        assert_eq!(Solution::coin_change_rust_way(&[1, 2, 5], 11), Some(3));

        let coins = vec![2];
        let amount = 3;
        let result = -1;
        assert_eq!(Solution::coin_change(coins, amount), result);
        assert_eq!(Solution::coin_change_rust_way(&[2], 3), None);

        let coins = vec![1];
        let amount = 0;
        let result = 0;
        assert_eq!(Solution::coin_change(coins, amount), result);
        assert_eq!(Solution::coin_change_rust_way(&[1], 0), Some(0));
    }
}
