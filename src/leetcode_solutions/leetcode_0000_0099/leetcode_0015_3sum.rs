use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut result: Vec<Vec<i32>> = vec![];

        if n < 3 {
            return result;
        }

        nums.sort_unstable();

        for i in 0..n - 2 {
            if nums[i] > 0 {
                break;
            }

            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut j = i + 1;
            let mut k = n - 1;
            while j < k {
                match (nums[i] + nums[j] + nums[k]).cmp(&0) {
                    Ordering::Equal => {
                        result.push(vec![nums[i], nums[j], nums[k]]);

                        j += 1;
                        k -= 1;

                        while j < k && nums[j] == nums[j - 1] {
                            j += 1;
                        }
                        while j < k && nums[k] == nums[k + 1] {
                            k -= 1;
                        }
                    }
                    Ordering::Greater => {
                        k -= 1;
                    }
                    Ordering::Less => {
                        j += 1;
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0015() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let result: Vec<Vec<i32>> = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(Solution::three_sum(nums), result);

        let nums = vec![0, 1, 1];
        let result: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::three_sum(nums), result);

        let nums = vec![0, 0, 0];
        let result: Vec<Vec<i32>> = vec![vec![0, 0, 0]];
        assert_eq!(Solution::three_sum(nums), result);
    }
}
