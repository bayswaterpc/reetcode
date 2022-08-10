use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn valid_sudoku_36_(board: Vec<Vec<char>>) -> bool {
        solutions::is_valid_sudoku(board)
    }
}

pub mod test {
    use crate::utils::test_utils::array2d_to_vec2d;
    #[allow(dead_code)]
    pub fn do_unit(
        board: [[char; 9]; 9],
        output: bool,
        is_valid_sudoku: fn(Vec<Vec<char>>) -> bool,
    ) {
        assert_eq!(output, is_valid_sudoku(array2d_to_vec2d(board)));
    }
}
