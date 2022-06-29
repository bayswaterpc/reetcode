use super::Solution;

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::default();
        for word in &words {
            let mut node = &mut trie;
            for u in word.bytes() {
                node = node.children[(u - b'a') as usize].get_or_insert(Default::default());
            }
            node.is_end = true;
        }

        let mut board = board;
        let mut answer = Vec::new();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                Self::backtrack(
                    &mut board,
                    &mut trie,
                    (i, j),
                    &mut String::new(),
                    &mut answer,
                );
            }
        }
        answer
    }

    fn backtrack(
        board: &mut Vec<Vec<char>>,
        trie: &mut Trie,
        pos: (usize, usize),
        s: &mut String,
        answer: &mut Vec<String>,
    ) {
        let c = board[pos.0][pos.1];
        if let Some(node) = &mut trie.children[(c as u8 - b'a') as usize] {
            s.push(c);
            if node.is_end {
                answer.push(s.clone());
                node.is_end = false;
            }
            board[pos.0][pos.1] = '*';
            for d in [0, 1, 0, !0, 0].windows(2) {
                let i = pos.0.wrapping_add(d[0]);
                let j = pos.1.wrapping_add(d[1]);
                if (0..board.len()).contains(&i)
                    && (0..board[0].len()).contains(&j)
                    && board[i][j] != '*'
                {
                    Self::backtrack(board, node, (i, j), s, answer);
                }
            }
            board[pos.0][pos.1] = c;
            s.pop();
        }
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;
    #[allow(unused_imports)]
    use crate::utils::test_utils::build_tree_from_lvl_order_str;
    use std::collections::HashSet;

    pub fn test_word_search_ii(board: Vec<Vec<char>>, words: &[&str], output: &[&str]) {
        let words = words
            .iter()
            .map(|s| (*s).to_string())
            .collect::<Vec<String>>();
        let output = output
            .iter()
            .map(|s| (*s).to_string())
            .collect::<Vec<String>>();

        let expected_set: HashSet<String> = HashSet::from_iter(output);
        let out_set: HashSet<String> = HashSet::from_iter(Solution::find_words(board, words));
        assert_eq!(expected_set, out_set);
    }

    #[test]
    fn unit() {
        let board = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        let words = ["oath", "pea", "eat", "rain"];
        let output = ["eat", "oath"];
        test_word_search_ii(board, &words, &output);

        let board = vec![
            vec!['o', 'a', 'b', 'n'],
            vec!['o', 't', 'a', 'e'],
            vec!['a', 'h', 'k', 'r'],
            vec!['a', 'f', 'l', 'v'],
        ];
        let words = ["oa", "oaa"];
        let output = ["oa", "oaa"];
        test_word_search_ii(board, &words, &output);
    }
}
