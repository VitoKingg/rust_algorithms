struct Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let s1: Vec<i32> = num1.bytes().map(|x| (x - b'0') as i32).rev().collect();
        let s2: Vec<i32> = num2.bytes().map(|x| (x - b'0') as i32).rev().collect();
        let mut s3: Vec<char> = vec![];

        let n1 = s1.len();
        let n2 = s2.len();
        let mut carry = 0;
        let mut i = 0;
        while i < n1 || i < n2 || carry > 0 {
            let mut v = 0;

            if i < n1 {
                v += s1[i];
            }

            if i < n2 {
                v += s2[i];
            }

            v += carry;
            i += 1;
            carry = v / 10;
            s3.push(((v % 10) as u8 + b'0') as char);
        }

        let result: String = s3.iter().rev().collect();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0415() {
        let num1 = String::from("11");
        let num2 = String::from("123");
        let result = String::from("134");
        assert_eq!(Solution::add_strings(num1, num2), result);

        let num1 = String::from("456");
        let num2 = String::from("77");
        let result = String::from("533");
        assert_eq!(Solution::add_strings(num1, num2), result);

        let num1 = String::from("0");
        let num2 = String::from("0");
        let result = String::from("0");
        assert_eq!(Solution::add_strings(num1, num2), result);
    }
}
