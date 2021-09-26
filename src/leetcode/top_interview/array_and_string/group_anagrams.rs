use std::collections::HashMap;

// 49. Group Anagrams, Medium
// https://leetcode.com/problems/group-anagrams/
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut ans: Vec<Vec<String>> = Vec::new();
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for str in strs {
            let mut chars = str.chars().collect::<Vec<char>>();
            chars.sort();
            let key: String = chars.into_iter().collect();
            if let Some(v) = map.get_mut(&key) {
                v.push(str);
            } else {
                map.insert(key, vec![str]);
            }
        }

        for (_, v) in map {
            ans.push(v);
        }
        return ans;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vec_string, vec_vec_i32, vec_vec_string};

    #[test]
    fn test_group_anagrams() {
        assert_eq!(Solution::group_anagrams(vec_string!["eat", "tea", "tan", "ate", "nat", "bat"]).len(), 3);
    }

    #[test]
    fn test_group_anagrams2() {
        assert_eq!(Solution::group_anagrams(vec_string![""]), vec_vec_string![[""]]);
    }

    #[test]
    fn test_group_anagrams3() {
        assert_eq!(Solution::group_anagrams(vec_string!["a"]), vec_vec_string![["a"]]);
    }
}
