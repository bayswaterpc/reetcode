use super::Solution;
use std::collections::HashMap;

impl Solution {
    /// Solution to [Best Time to Buy and Sell Stock](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/)
    // in leetcode use below def
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut res: Vec<Vec<String>> = vec![];
        let mut hash_to_anagram_group: HashMap<[u8; 26], usize> = HashMap::new();

        for str in strs {
            let mut c_count = [0; 26];
            for c in str.chars() {
                let ci = c as usize - 'a' as usize;
                c_count[ci] += 1;
            }

            match hash_to_anagram_group.get(&c_count) {
                Some(ii) => res[*ii].push(str),
                None => {
                    hash_to_anagram_group.insert(c_count, res.len());
                    res.push(vec![str])
                }
            }
        }

        res
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;
    #[allow(dead_code, unused_imports)]
    use crate::utils::test_utils::{
        str_vec_2d_contents_same, str_vec_2d_to_string_vec_2d, str_vec_to_string_vec,
    };

    #[test]
    fn unit() {
        let strs = str_vec_to_string_vec(vec!["eat", "tea", "tan", "ate", "nat", "bat"]);
        let ans = str_vec_2d_to_string_vec_2d(vec![
            vec!["ate", "eat", "tea"],
            vec!["tan", "nat"],
            vec!["bat"],
        ]);
        assert!(str_vec_2d_contents_same(
            ans,
            Solution::group_anagrams(strs)
        ));

        let strs = str_vec_to_string_vec(vec![""]);
        let ans = str_vec_2d_to_string_vec_2d(vec![vec![""]]);
        assert!(str_vec_2d_contents_same(
            ans,
            Solution::group_anagrams(strs)
        ));

        let strs = str_vec_to_string_vec(vec!["a"]);
        let ans = str_vec_2d_to_string_vec_2d(vec![vec!["a"]]);
        assert!(str_vec_2d_contents_same(
            ans,
            Solution::group_anagrams(strs)
        ));
    }
}
