// tower of hanoi 汉诺塔
pub fn tower_of_hanoi(n: i32, from: i32, to: i32, via: i32, moves: &mut Vec<(i32, i32)>) {
    if n > 0 {
        tower_of_hanoi(n - 1, from, via, to, moves);
        moves.push((from, to));
        tower_of_hanoi(n - 1, via, to, from, moves);
    }
}

#[cfg(test)]
mod math_tests {
    use super::tower_of_hanoi;

    #[test]
    fn test_tower_of_hanoi() {
        let mut curr_result: Vec<(i32, i32)> = Vec::new();
        let expect_result = vec![(1, 3), (1, 2), (3, 2), (1, 3), (2, 1), (2, 3), (1, 3)];
        tower_of_hanoi(3, 1, 3, 2, &mut curr_result);
        assert_eq!(expect_result, curr_result);
    }
}
