#![allow(unused_variables)]
#![allow(clippy::ptr_arg)]

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn exit(board: Vec<Vec<char>>, word: String) -> bool {
        let m = board.len();
        let n = board[0].len();
        let word: Vec<char> = word.chars().collect();
        let mut visited = vec![vec![false; n]; m];
        let mut q: VecDeque<(usize, usize, bool)> = VecDeque::new();

        for i in 0..m {
            for j in 0..n {
                if Self::dfs(i, j, 0, &word, &board, &mut visited, &mut q) {
                    return true;
                }
            }
        }

        false
    }

    fn dfs(
        i: usize,
        j: usize,
        index: usize,
        word: &[char],
        board: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        queue: &mut VecDeque<(usize, usize, bool)>,
    ) -> bool {
        if board[i][j] != word[index] {
            return false;
        }

        if index == word.len() - 1 {
            return true;
        }

        visited[i][j] = true;
        // todo: VecDeque<(usize, usize, bool)>, 参考 LeetCode 0542

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0079() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = String::from("ABCCED");
        assert!(Solution::exit(board, word));

        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = String::from("SEE");
        assert!(Solution::exit(board, word));

        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = String::from("ABCB");
        assert!(!Solution::exit(board, word));
    }
}
