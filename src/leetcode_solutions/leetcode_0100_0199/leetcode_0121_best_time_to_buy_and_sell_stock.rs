struct Solution;

impl Solution {
    /// T(n) = O(n)
    /// S(n) = O(1)
    /// dp solution
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut cost = i32::MAX;

        for price in prices {
            profit = profit.max(price - cost);
            cost = cost.min(price);
        }

        profit
    }

    /// T(n) = O(n ^ 2)
    /// S(n) = O(1)
    /// iteration solution: time limit exceeded
    pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        let mut profit: i32 = 0;

        for i in 0..len {
            for j in i..len {
                let temp: i32 = prices[j] - prices[i];
                if temp > 0 {
                    profit = profit.max(temp);
                }
            }
        }

        profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0121() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let profit = 5;
        assert_eq!(Solution::max_profit(prices), profit);

        let prices = vec![7, 6, 4, 3, 1];
        let profit = 0;
        assert_eq!(Solution::max_profit(prices), profit);
    }
}
