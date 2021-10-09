use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

// 212. Word Search II, Hard
// https://leetcode.com/problems/word-search-ii/
impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        struct WordTie {
            char: char,
            is_word: bool,
            children: HashMap<char, WordTie>,
        }

        impl WordTie {
            fn new(char: char) -> Self {
                WordTie {
                    char,
                    is_word: false,
                    children: HashMap::new(),
                }
            }
        }

        let mut word_tie = WordTie::new('/');
        for word in words.iter() {
            let mut current = &mut word_tie;
            for c in word.chars() {
                current.children.entry(c).or_insert(WordTie::new(c));
                current = current.children.get_mut(&c).unwrap();
            }
            current.is_word = true;
        }

        fn backtracking(board: &mut Vec<Vec<char>>, word_tie: &WordTie, ans: &mut HashSet<String>, curr: String, i: i32, j: i32) {
            let (n, m) = (board.len() as i32, board[0].len() as i32);
            let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

            if word_tie.is_word {
                ans.insert(curr.clone());
            }

            for dir in dirs {
                let (di, dj) = (i + dir.0, j + dir.1);
                if 0 > di || n <= di || 0 > dj || m <= dj {
                    continue;
                }

                let char = board[di as usize][dj as usize];

                if char == '.' {
                    continue;
                }

                if word_tie.children.contains_key(&char) {
                    let mut next_curr = curr.clone();
                    next_curr.push_str(char.to_string().as_str());
                    board[di as usize][dj as usize] = '.';
                    backtracking(board, word_tie.children.get(&char).unwrap(), ans, next_curr, di, dj);
                    board[di as usize][dj as usize] = char;
                }
            }
        }

        let mut ans: HashSet<String> = HashSet::new();
        let mut board = board;
        let (n, m) = (board.len(), board[0].len());
        for i in 0..n {
            for j in 0..m {
                let char = board[i][j];
                if word_tie.children.contains_key(&char) {
                    board[i][j] = '.';
                    backtracking(&mut board, word_tie.children.get(&char).unwrap(), &mut ans, char.to_string(), i as i32, j as i32);
                    board[i][j] = char;
                }
            }
        }

        Vec::from_iter(ans)
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vec_string, vec_vec_char};

    #[test]
    fn test_find_words() {
        assert_eq!(
            Solution::find_words(
                vec_vec_char![['o', 'a', 'a', 'n'], ['e', 't', 'a', 'e'], ['i', 'h', 'k', 'r'], ['i', 'f', 'l', 'v']],
                vec_string!["oath", "pea", "eat", "rain"]
            )
            .len(),
            2
        );
    }

    #[test]
    fn test_find_words2() {
        assert_eq!(Solution::find_words(vec_vec_char![['a', 'b'], ['c', 'd']], vec_string!["abcb"]), vec_string![]);
    }

    #[test]
    fn test_find_words3() {
        assert_eq!(Solution::find_words(vec_vec_char![['a']], vec_string!["a"]), vec_string!["a"]);
    }

    #[test]
    fn test_find_words4() {
        assert_eq!(
            Solution::find_words(
                vec_vec_char![['o', 'a', 'b', 'n'], ['o', 't', 'a', 'e'], ['a', 'h', 'k', 'r'], ['a', 'f', 'l', 'v']],
                vec_string!["oa", "oaa"]
            )
            .len(),
            2
        );
    }

    #[test]
    fn test_find_words5() {
        assert_eq!(Solution::find_words(vec_vec_char![['a', 'a']], vec_string!["aaa"]), vec_string![]);
    }
}
