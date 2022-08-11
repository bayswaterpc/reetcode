pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    matrix.concat().binary_search(&target).is_ok()
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::search_a_2d_matrix_74_::test::do_unit;

    #[test]
    fn unit() {
        do_unit(
            [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]],
            3,
            true,
            super::search_matrix,
        );
        do_unit(
            [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]],
            13,
            false,
            super::search_matrix,
        );
    }
}
