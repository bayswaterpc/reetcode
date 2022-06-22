use super::Solution;

impl Solution {
    /// [Shortest Word Distance](https://leetcode.com/problems/shortest-word-distance/)
    pub fn shortest_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
        let (mut j1, mut j2) = (-1, -1);
        let mut res = words_dict.len() as i32;
        for (ii, word) in words_dict.iter().enumerate() {
            if word1 == *word {
                j1 = ii as i32;
            } else if word2 == *word {
                j2 = ii as i32;
            }

            if j1 != -1 && j2 != -1 {
                res = std::cmp::min(res, (j1 - j2).abs())
            }
        }
        res as i32
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let words_dict = vec!["small".to_string(), "big".to_string()];
        let word1 = "small".to_string();
        let word2 = "big".to_string();
        let i = Solution::shortest_distance(words_dict, word1, word2);
        assert_eq!(1, i);
    }
}
