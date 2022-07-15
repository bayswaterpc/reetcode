enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut down_bound = matrix.len() - 1;
    let mut up_bound = 0;
    let mut right_bound = matrix[0].len() - 1;
    let mut left_bound = 0;

    let total = matrix.len() * matrix[0].len();

    let mut i = 0;
    let mut j = 0;
    let mut seen = 0;
    let mut direction = Direction::Right;
    let mut res: Vec<i32> = vec![];

    while seen < total {
        res.push(matrix[i][j]);
        match direction {
            Direction::Right => {
                if j == right_bound {
                    direction = Direction::Down;
                    i += 1;
                    up_bound += 1;
                } else {
                    j += 1;
                }
            }
            Direction::Left => {
                if j == left_bound {
                    direction = Direction::Up;
                    i -= 1;
                    down_bound -= 1;
                } else {
                    j -= 1;
                }
            }
            Direction::Up => {
                if i == up_bound {
                    direction = Direction::Right;
                    j += 1;
                    left_bound += 1;
                } else {
                    i -= 1;
                }
            }
            Direction::Down => {
                if i == down_bound {
                    direction = Direction::Left;
                    j -= 1;
                    right_bound -= 1;
                } else {
                    i += 1;
                }
            }
        }
        seen += 1;
    }

    res
}

pub mod test {
    #[allow(unused_imports, clippy::ptr_arg)]
    use crate::problems::spiral_matrix::test::do_unit;

    #[test] //replace allow with test when ready
            // Use VsCode run or debug options or..
            // change name to a unique test name and run `cargo test unq_name` or  run ..
            // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit(
            [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
            [1, 2, 3, 6, 9, 8, 7, 4, 5],
            super::spiral_order,
        );
    }
}
