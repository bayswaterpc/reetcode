use super::Solution;
impl Solution {
    ///Solution to vec![Word Search](https://leetcode.com/problems/word-search/)
    // pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    pub fn word_search(board: Vec<Vec<char>>, word: String) -> bool {
        fn dfs(
            board: &[Vec<char>],
            word: &[char],
            visited: &mut [Vec<bool>],
            ii: usize,
            jj: usize,
            depth: usize,
        ) -> bool {
            depth == word.len()
                || (ii < board.len()
                    && jj < board[0].len()
                    && !visited[ii][jj]
                    && word[depth] == board[ii][jj]
                    && {
                        visited[ii][jj] = true;
                        let rez = [0, 1, 0, !0, 0].windows(2).any(|w| {
                            dfs(
                                board,
                                word,
                                visited,
                                ii.wrapping_add(w[0]),
                                jj.wrapping_add(w[1]),
                                depth + 1,
                            )
                        });
                        visited[ii][jj] = false;
                        rez
                    })
        }

        let m = board.len();
        let n = board[0].len();
        let word = word.chars().collect::<Vec<_>>();
        let mut visited = vec![vec![false; n]; m];

        (0..m).any(|i| (0..n).any(|j| dfs(&board, &word, &mut visited, i, j, 0)))
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCCED";
        let output = true;
        assert_eq!(output, Solution::word_search(board, word.to_string()));

        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "SEE";
        let output = true;
        assert_eq!(output, Solution::word_search(board, word.to_string()));

        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCB";
        let output = false;
        assert_eq!(output, Solution::word_search(board, word.to_string()));
    }
}
