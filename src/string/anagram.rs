//! [Anagram Wikipedia](https://en.wikipedia.org/wiki/Anagram)
//! An anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
//! typically using all the original letters exactly once.
//! anagram 变位词，乱序字符串：改变某个词或短语的字母顺序后构成的新词或短语

pub fn anagram(s1: &str, s2: &str) -> bool {
    let mut c1 = [0; 26];
    let mut c2 = [0; 26];

    generate_ascii_array(s1, &mut c1);
    generate_ascii_array(s2, &mut c2);

    let mut pos = 0;
    let mut equal = true;
    while pos < 26 && equal {
        if c1[pos] == c2[pos] {
            pos += 1;
        } else {
            equal = false;
        }
    }

    equal
}

fn generate_ascii_array(s: &str, arr: &mut [usize]) {
    for ch in s.to_ascii_lowercase().chars() {
        if ('a'..='z').contains(&ch) {
            let pos = (ch as usize) - 97; // 97 为 a 的 ASCII 值
            arr[pos] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::anagram;

    #[test]
    fn anagram_test() {
        let sa1 = "coronavirus";
        let sb1 = "carnivorous";
        assert!(anagram(sa1, sb1));

        let sa2 = "funeral";
        let sb2 = "real fun";
        assert!(anagram(sa2, sb2));

        let sa3 = "New York Times";
        let sb3 = "monkeys write";
        assert!(anagram(sa3, sb3));

        let sa4 = "Church of Scientology";
        let sb4 = "rich-chosen goofy cult";
        assert!(anagram(sa4, sb4));

        let sa5 = "William Shakespeare";
        let sb5 = "I am a weakish speller";
        assert!(anagram(sa5, sb5));
    }
}
