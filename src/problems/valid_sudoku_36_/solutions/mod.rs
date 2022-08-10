use std::collections::HashSet;

fn get_sub_box_indx(i: usize, j: usize) -> usize {
    i / 3 * 3 + j / 3
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut row_set = vec![HashSet::new(); 9];
    let mut col_set = vec![HashSet::new(); 9];
    let mut box_set = vec![HashSet::new(); 9];
    for i in 0..9 {
        for j in 0..9 {
            let c = board[i][j];
            if c == '.' {
                continue;
            }
            let c = c.to_digit(10).unwrap() as usize;
            if row_set[i].contains(&c) {
                return false;
            }
            row_set[i].insert(c);
            if col_set[j].contains(&c) {
                return false;
            }
            col_set[j].insert(c);
            if box_set[get_sub_box_indx(i, j)].contains(&c) {
                return false;
            }
            box_set[get_sub_box_indx(i, j)].insert(c);
        }
    }
    true
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::valid_sudoku_36_::test::do_unit;

    #[test]
    fn unit() {
        let board = [
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        do_unit(board, true, super::is_valid_sudoku);

        let board = [
            ['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        do_unit(board, false, super::is_valid_sudoku);
    }
}
