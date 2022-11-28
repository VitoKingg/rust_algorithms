/// [edit distance](https://en.wikipedia.org/wiki/Edit_distance)
struct Solution;

impl Solution {
    // T(n) = O(m * n)
    // S(n) = O(m * n)
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let m = word1.len();
        let n = word2.len();
        let word1: Vec<char> = word1.chars().collect();
        let word2: Vec<char> = word2.chars().collect();
        let mut dp = vec![vec![0; n + 1]; m + 1];

        // for i in 0..=m {
        //     dp[i][0] = i;
        // }

        // for j in 0..=n {
        //     dp[0][j] = j;
        // }

        for (i, item) in dp.iter_mut().enumerate() {
            item[0] = i;
        }

        for (j, item) in dp[0].iter_mut().enumerate() {
            *item = j;
        }

        for i in 1..=m {
            for j in 1..=n {
                if word1[i - 1] == word2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
                }
            }
        }

        dp[m][n] as i32
    }

    // T(n) = O(m * n)
    // S(n) = O(m * n)
    pub fn min_distance2(word1: String, word2: String) -> i32 {
        let m = word1.chars().count();
        let n = word2.chars().count();
        let mut dp = vec![vec![0; n + 1]; m + 1];

        // for i in 0..=m {
        //     dp[i][0] = i;
        // }

        // for j in 0..=n {
        //     dp[0][j] = j;
        // }

        for (i, item) in dp.iter_mut().enumerate() {
            item[0] = i;
        }

        for (j, item) in dp[0].iter_mut().enumerate() {
            *item = j;
        }

        for (i, c1) in word1.chars().enumerate() {
            for (j, c2) in word2.chars().enumerate() {
                let insert_count = dp[i + 1][j] + 1;
                let delete_count = dp[i][j + 1] + 1;
                let replace_count = dp[i][j] + (c1 != c2) as usize;

                dp[i + 1][j + 1] = insert_count.min(delete_count).min(replace_count);
            }
        }

        dp[m][n] as i32
    }

    // T(n) = O(m * n)
    // S(n) = O(min(m, n))
    pub fn min_distance3(word1: String, word2: String) -> i32 {
        let n = word2.chars().count();
        let mut dp = (0..=n).collect::<Vec<_>>();

        for (i, c1) in word1.chars().enumerate() {
            let mut replace_count = i;
            dp[0] = replace_count + 1;

            for (j, c2) in word2.chars().enumerate() {
                let distance = (dp[j].min(dp[j + 1]) + 1).min(replace_count + (c1 != c2) as usize);
                replace_count = dp[j + 1];
                dp[j + 1] = distance;
            }
        }

        *dp.last().unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_leetcode_0072_1() {
        let word1 = String::from("horse");
        let word2 = String::from("ros");
        let res = 3;
        assert_eq!(Solution::min_distance(word1, word2), res);

        let word1 = String::from("intention");
        let word2 = String::from("execution");
        let res = 5;
        assert_eq!(Solution::min_distance(word1, word2), res);

        let word1 = String::from("");
        let word2 = String::from("empty");
        let res = 5;
        assert_eq!(Solution::min_distance(word1, word2), res);

        let word1 = String::from("empty");
        let word2 = String::from("");
        let res = 5;
        assert_eq!(Solution::min_distance(word1, word2), res);
    }

    #[test]
    fn test_leetcode_0072_2() {
        let word1 = String::from("horse");
        let word2 = String::from("ros");
        let res = 3;
        assert_eq!(Solution::min_distance2(word1, word2), res);

        let word1 = String::from("intention");
        let word2 = String::from("execution");
        let res = 5;
        assert_eq!(Solution::min_distance2(word1, word2), res);

        let word1 = String::from("");
        let word2 = String::from("empty");
        let res = 5;
        assert_eq!(Solution::min_distance2(word1, word2), res);

        let word1 = String::from("empty");
        let word2 = String::from("");
        let res = 5;
        assert_eq!(Solution::min_distance2(word1, word2), res);
    }

    #[test]
    fn test_leetcode_0072_3() {
        let word1 = String::from("horse");
        let word2 = String::from("ros");
        let res = 3;
        assert_eq!(Solution::min_distance3(word1, word2), res);

        let word1 = String::from("intention");
        let word2 = String::from("execution");
        let res = 5;
        assert_eq!(Solution::min_distance3(word1, word2), res);

        let word1 = String::from("");
        let word2 = String::from("empty");
        let res = 5;
        assert_eq!(Solution::min_distance3(word1, word2), res);

        let word1 = String::from("empty");
        let word2 = String::from("");
        let res = 5;
        assert_eq!(Solution::min_distance3(word1, word2), res);
    }
}
