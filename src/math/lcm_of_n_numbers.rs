/// returns the least common multiple of n numbers

pub fn lcm_of_n_numbers(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }

    let a = nums[0];
    let b = lcm_of_n_numbers(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod math_tests {
    use super::lcm_of_n_numbers;

    #[test]
    fn lcm_of_n_numbers_test() {
        assert_eq!(lcm_of_n_numbers(&[1, 2, 3, 4, 5]), 60);
        assert_eq!(lcm_of_n_numbers(&[2, 4, 6, 8, 10]), 120);
        assert_eq!(lcm_of_n_numbers(&[3, 6, 9, 12, 15]), 180);
        assert_eq!(lcm_of_n_numbers(&[10]), 10);
        assert_eq!(lcm_of_n_numbers(&[21, 110]), 2310);
    }
}
