pub fn base_conversion(mut deci_num: u32, base_num: u32) -> String {
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];

    let mut remainder_stack = vec![];
    while deci_num > 0 {
        let remainder = deci_num % base_num;
        remainder_stack.push(remainder);
        deci_num /= base_num;
    }

    let mut base_str = String::from("");
    while let Some(remainder) = remainder_stack.pop() {
        base_str += &digits[remainder as usize].to_string();
    }

    base_str
}

#[cfg(test)]
mod tests {
    use super::base_conversion;

    #[test]
    fn test_base_conversion() {
        assert_eq!(base_conversion(10, 2), String::from("1010"));
        assert_eq!(base_conversion(43, 16), String::from("2B"));
        assert_eq!(base_conversion(121, 8), String::from("171"));
    }
}
