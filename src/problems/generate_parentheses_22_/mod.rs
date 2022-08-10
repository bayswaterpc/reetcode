use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    //lol there is disagreement in spelling between leetcodes function call and problem slug
    pub fn generate_parentheses_22_(n: i32) -> Vec<String> {
        solutions::generate_parenthesis(n)
    }
}

pub mod test {
    use crate::utils::test_utils::str_vec_to_string_vec;
    pub fn do_unit<const M: usize, T: Clone + PartialEq + std::fmt::Debug + Sized>(
        input: T,
        output: [&str; M],
        generate_parentheses: fn(T) -> Vec<String>,
    ) {
        assert_eq!(str_vec_to_string_vec(output.to_vec()), generate_parentheses(input));
    }
}
