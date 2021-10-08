// 1268. Search Suggestions System, Medium
// https://leetcode.com/problems/search-suggestions-system/
impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut suggested_products = vec![];
        for i in 0..search_word.len() {
            println!("{}", &search_word[..i + 1]);
            let mut filtered = products
                .iter()
                .filter(|p| p.starts_with(&search_word[..i + 1]))
                .map(|p| p.to_string())
                .collect::<Vec<String>>();
            filtered.sort();

            suggested_products.push(filtered.iter().take(3).cloned().collect::<Vec<String>>());
        }

        suggested_products
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vec_string, vec_vec_string};

    #[test]
    fn test_suggested_products() {
        assert_eq!(
            Solution::suggested_products(vec_string!["mobile", "mouse", "moneypot", "monitor", "mousepad"], "mouse".to_string()),
            vec_vec_string![
                ["mobile", "moneypot", "monitor"],
                ["mobile", "moneypot", "monitor"],
                ["mouse", "mousepad"],
                ["mouse", "mousepad"],
                ["mouse", "mousepad"]
            ]
        );
    }

    #[test]
    fn test_suggested_products2() {
        assert_eq!(
            Solution::suggested_products(vec_string!["havana"], "havana".to_string()),
            vec_vec_string![["havana"], ["havana"], ["havana"], ["havana"], ["havana"], ["havana"]]
        );
    }

    #[test]
    fn test_suggested_products3() {
        assert_eq!(
            Solution::suggested_products(vec_string!["bags", "baggage", "banner", "box", "cloths"], "bags".to_string()),
            vec_vec_string![["baggage", "bags", "banner"], ["baggage", "bags", "banner"], ["baggage", "bags"], ["bags"]]
        );
    }

    #[test]
    fn test_suggested_products4() {
        assert_eq!(
            Solution::suggested_products(vec_string!["havana"], "tatiana".to_string()),
            vec_vec_string![[], [], [], [], [], [], []]
        );
    }
}
