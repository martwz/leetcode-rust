use std::collections::HashMap;

// 208. Implement Trie (Prefix Tree), Medium
// https://leetcode.com/problems/implement-trie-prefix-tree/
struct CharNode {
    char: char,
    is_word: bool,
    children: HashMap<char, CharNode>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: CharNode {
                char: '*',
                is_word: false,
                children: HashMap::new(),
            },
        }
    }

    fn insert(&mut self, word: String) {
        let mut current = &mut self.root;
        for c in word.chars() {
            if !current.children.contains_key(&c) {
                current.children.insert(
                    c,
                    CharNode {
                        char: c,
                        is_word: false,
                        children: HashMap::new(),
                    },
                );
            }
            current = current.children.get_mut(&c).unwrap();
        }
        current.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut current = &self.root;
        for c in word.chars() {
            if !current.children.contains_key(&c) {
                return false;
            }
            current = current.children.get(&c).unwrap();
        }

        current.is_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut current = &self.root;
        for c in prefix.chars() {
            if !current.children.contains_key(&c) {
                return false;
            }
            current = current.children.get(&c).unwrap();
        }
        true
    }
}

struct Trie {
    root: CharNode,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut obj = Trie::new();
        obj.insert("Martin".to_string());

        let ret_2: bool = obj.search("Martin".to_string());
        assert_eq!(ret_2, true);

        let ret_3: bool = obj.starts_with("Mar".to_string());
        assert_eq!(ret_3, true);

        let ret_4: bool = obj.starts_with("Mara".to_string());
        assert_eq!(ret_4, false);

        let ret_5: bool = obj.search("Mar".to_string());
        assert_eq!(ret_5, false);
    }
}
