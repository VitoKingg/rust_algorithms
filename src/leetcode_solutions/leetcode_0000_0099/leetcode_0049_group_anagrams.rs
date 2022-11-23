use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result = vec![];
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for str in strs {
            let mut temp_string: Vec<char> = str.chars().collect();
            temp_string.sort_unstable();
            let key_string: String = temp_string.iter().map(|c| c.to_string()).collect();

            map.entry(key_string)
                .and_modify(|v| v.push(String::from(&str)))
                .or_insert_with(|| vec![String::from(&str)]);

            // another method:
            // let group = map.entry(key_string).or_default();
            // group.push(str);
        }

        for (_, v) in map {
            result.push(v);
        }

        result.sort_by_key(|x| x.len());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_0049() {
        let strs = vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ];
        let _result = vec![
            vec![String::from("bat")],
            vec![String::from("nat"), String::from("tan")],
            vec![
                String::from("ate"),
                String::from("eat"),
                String::from("tea"),
            ],
        ];
        println!("{:#?}", Solution::group_anagrams(strs));

        let strs = vec![String::from("")];
        let _result = vec![vec![String::from("")]];
        println!("{:#?}", Solution::group_anagrams(strs));

        let strs = vec![String::from("a")];
        let _result = vec![vec![String::from("a")]];
        println!("{:#?}", Solution::group_anagrams(strs));
    }
}
