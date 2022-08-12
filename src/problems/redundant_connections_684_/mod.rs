use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn redundant_connections_684_(edges: Vec<Vec<i32>>) -> Vec<i32> {
        solutions::find_redundant_connection(edges)
    }
}

pub mod test {
    use crate::utils::test_utils::array2d_to_vec2d;
    pub fn do_unit<const N: usize, T: Clone + PartialEq + std::fmt::Debug + Sized>(
        edges: [[T; 2]; N],
        output: [T; 2],
        f: fn(Vec<Vec<T>>) -> Vec<T>,
    ) {
        assert_eq!(output.to_vec(), f(array2d_to_vec2d(edges)));
    }
}
