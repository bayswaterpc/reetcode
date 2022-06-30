use super::Solution;
impl Solution {
    ///Solution to vec![Word Search](https://leetcode.com/problems/word-search/)
    // pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    pub fn word_search(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board;
        fn backtrack(
            board: &mut Vec<Vec<char>>,
            combo: &mut String,
            word: &String,
            ii: usize,
            jj: usize,
        ) -> bool {
            println!("{} {}", combo, word);
            if *combo == *word {
                println!("-------------- should return true!!!!!");
                return true;
            }

            if board[ii][jj] != word.chars().nth(combo.len()).unwrap() {
                return false;
            }
            let c = board[ii][jj];
            board[ii][jj] = '#';
            combo.push(c);

            let mut test_dirs = vec![];

            // East
            if ii < board.len() - 1 {
                test_dirs.push((ii+1, jj));
            }
            // South
            if jj < board[0].len() - 1 {
                test_dirs.push((ii, jj+1));
            }
            // West
            if ii > 0 {
                test_dirs.push((ii-1, jj));

            }
            // North
            if jj > 0 {
                test_dirs.push((ii, jj-1));
            }

            for (ni, nj) in test_dirs.iter() {
                if backtrack(board, combo, word, *ni, *nj) {
                    println!(" returned true please stop.......");
                    return true;
                }
            }
            
            combo.pop();
            board[ii][jj] = c;
            false
        }

        let mut combo = "".to_string();
        for ii in 0..board.len() {
            for jj in 0..board[0].len() {
                if backtrack(&mut board, &mut combo, &word, ii, jj) {
                    println!(" plz.....");
                    return true;
                }
            }
        }
        false
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
