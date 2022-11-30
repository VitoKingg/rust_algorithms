#![allow(clippy::needless_range_loop)]

use std::collections::VecDeque;

struct Solution;

impl Solution {
    /// BFS with queue
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let x = mat.len();
        let y = mat[0].len();
        let mut mat = mat;
        let mut visited: Vec<Vec<bool>> = vec![vec![false; y]; x];
        let mut q: VecDeque<(usize, usize, i32)> = VecDeque::new();

        for i in 0..x {
            for j in 0..y {
                if mat[i][j] == 0 {
                    q.push_back((i, j, 0));
                }
            }
        }

        while let Some((i, j, d)) = q.pop_front() {
            if visited[i][j] {
                continue;
            }

            visited[i][j] = true;
            mat[i][j] = d;

            if i > 0 {
                q.push_back((i - 1, j, d + 1));
            }

            if j > 0 {
                q.push_back((i, j - 1, d + 1));
            }

            if i + 1 < x {
                q.push_back((i + 1, j, d + 1));
            }

            if j + 1 < y {
                q.push_back((i, j + 1, d + 1));
            }
        }

        mat
    }

    /// DP solution
    pub fn update_matrix_dp(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let x = mat.len();
        let y = mat[0].len();
        let mut result: Vec<Vec<i32>> = vec![vec![i32::MAX - 100000; y]; x];

        // first pass: check left and top
        for i in 0..x {
            for j in 0..y {
                if mat[i][j] == 0 {
                    result[i][j] = 0;
                } else {
                    if i > 0 {
                        result[i][j] = result[i][j].min(result[i - 1][j] + 1);
                    }

                    if j > 0 {
                        result[i][j] = result[i][j].min(result[i][j - 1] + 1);
                    }
                }
            }
        }

        // second pass: check right and bottom
        for i in (0..x).rev() {
            for j in (0..y).rev() {
                if i + 1 < x {
                    result[i][j] = result[i][j].min(result[i + 1][j] + 1);
                }

                if j + 1 < y {
                    result[i][j] = result[i][j].min(result[i][j + 1] + 1);
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
    fn test_leetcode_0542() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let result = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution::update_matrix(mat), result);

        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        let result = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]];
        assert_eq!(Solution::update_matrix(mat), result);
    }
}
