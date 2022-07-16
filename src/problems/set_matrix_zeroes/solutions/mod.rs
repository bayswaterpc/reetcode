//time=O(M*N)
//time=O(1)
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    let m = matrix[0].len();

    let mut first_col_zeroes = false;

    for ii in 0..n {
        if matrix[ii][0] == 0 {
            first_col_zeroes = true;
        }
        for jj in 1..m {
            if matrix[ii][jj] == 0 {
                matrix[ii][0] = 0;
                matrix[0][jj] = 0;
            }
        }
    }

    for ii in 1..n {
        for jj in 1..m {
            if matrix[0][jj] == 0 || matrix[ii][0] == 0 {
                matrix[ii][jj] = 0;
            }
        }
    }

    if matrix[0][0] == 0 {
        for jj in 1..m {
            matrix[0][jj] = 0;
        }
    }

    if first_col_zeroes {
        for row in matrix.iter_mut() {
            row[0] = 0;
        }
    }
}

pub mod test {
    #[allow(unused_imports, clippy::ptr_arg)]
    use crate::problems::set_matrix_zeroes::test::do_unit;

    #[test]
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        // do_unit(
        //     [[1,1,1],[1,0,1],[1,1,1]],
        //     [[1,0,1],[0,0,0],[1,0,1]],
        //     super::set_zeroes,
        // );

        // do_unit(
        //     [[0,1,2,0],[3,4,5,2],[1,3,1,5]],
        //     [[0,0,0,0],[0,4,5,0],[0,3,1,0]],
        //     super::set_zeroes,
        // );

        do_unit([[1], [0]], [[0], [0]], super::set_zeroes);
    }
}
