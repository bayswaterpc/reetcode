use super::Solution;
use std::collections::HashSet;

impl Solution {
    /// Solution to [Word Break](https://leetcode.com/problems/word-break/)
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        fn word_break_memo(
            s: &str,
            start: usize,
            invalid_res_found: &mut [bool],
            dict_set: &HashSet<String>,
        ) -> bool {
            if start == s.len() {
                return true;
            }
            if invalid_res_found[start] {
                return false;
            }

            for end in (start + 1)..(s.len() + 1) {
                if dict_set.contains(&s[start..end])
                    && word_break_memo(s, end, invalid_res_found, dict_set)
                {
                    return true;
                }
            }
            invalid_res_found[start] = true;
            false
        }

        let mut invalid_res_found = vec![false; s.len()];
        let dict_set: HashSet<String> = HashSet::from_iter(word_dict.into_iter());

        word_break_memo(&s, 0, &mut invalid_res_found, &dict_set)
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        // let s = "leetcode".to_string();
        // let word_dict = vec!["leet".to_string(), "code".to_string()];
        // let output = true;
        // assert_eq!(output, Solution::word_break(s, word_dict));

        // let s = "applepenapple".to_string();
        // let word_dict = vec!["apple".to_string(), "pen".to_string()];
        // let output = true;
        // assert_eq!(output, Solution::word_break(s, word_dict));

        // let s = "catsandog".to_string();
        // let word_dict = vec![
        //     "cats".to_string(),
        //     "dog".to_string(),
        //     "sand".to_string(),
        //     "and".to_string(),
        //     "cat".to_string(),
        // ];
        // let output = false;
        // assert_eq!(output, Solution::word_break(s, word_dict));

        let s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string();
        let word_dict = vec![
            "a".to_string(),
            "aa".to_string(),
            "aaa".to_string(),
            "aaaa".to_string(),
            "aaaaa".to_string(),
            "aaaaaa".to_string(),
            "aaaaaaa".to_string(),
            "aaaaaaaa".to_string(),
            "aaaaaaaaa".to_string(),
            "aaaaaaaaaa".to_string(),
        ];
        let output = false;
        assert_eq!(output, Solution::word_break(s, word_dict));
    }
}
