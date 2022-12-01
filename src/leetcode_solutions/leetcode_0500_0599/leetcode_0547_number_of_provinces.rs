struct Solution;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut m = is_connected;
        let n = m.len();
        let mut visited: Vec<bool> = vec![false; n];

        for i in 0..n {
            if !visited[i] {
                visited[i] = true;
                Self::dfs(&mut m, &mut visited, i, n);
                result += 1;
            }
        }

        result
    }

    fn dfs(m: &mut Vec<Vec<i32>>, visited: &mut Vec<bool>, i: usize, n: usize) {
        for j in 0..n {
            if m[i][j] == 1 && !visited[j] {
                visited[j] = true;
                Self::dfs(m, visited, j, n);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0547() {
        let is_connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
        let result = 2;
        assert_eq!(Solution::find_circle_num(is_connected), result);

        let is_connected = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        let result = 3;
        assert_eq!(Solution::find_circle_num(is_connected), result);
    }
}
