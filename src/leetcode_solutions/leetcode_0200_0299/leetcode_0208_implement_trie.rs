/// [trie](https://en.wikipedia.org/wiki/Trie) 字典树、前缀树，应用于打字预测、自动补全、拼写检查等场景
/// [LeetCode 0208 Implement Trie (Prefix Tree) (Medium)](https://leetcode.com/problems/implement-trie-prefix-tree/)
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug, Default)]
struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut link = self;
        for c in word.chars() {
            link = link.children.entry(c).or_default();
        }
        link.end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut link = self;
        for c in word.chars() {
            if let Some(child) = link.children.get(&c) {
                link = child;
            } else {
                return false;
            }
        }
        link.end
    }
    fn starts_with(&self, word: String) -> bool {
        let mut link = self;
        for c in word.chars() {
            if let Some(child) = link.children.get(&c) {
                link = child;
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn test_leetcode_0208() {
        let mut trie = Trie::new();

        trie.insert(String::from("apple"));
        assert!(trie.search(String::from("apple")));
        assert!(!trie.search(String::from("app")));
        assert!(trie.starts_with(String::from("app")));

        trie.insert(String::from("app"));
        assert!(trie.search(String::from("app")));
    }
}
