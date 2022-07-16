use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn valid_palindrome_125_(s: String) -> bool {
        solutions::is_palindrome(s)
    }
}

pub mod test {
    use std::fmt::Debug;
    use std::string::ToString;
    #[allow(dead_code)]
    pub fn do_unit<T: Clone + Debug + PartialEq + ToString, U: Debug + PartialEq>(
        input: T,
        output: U,
        ff: fn(String) -> U,
    ) {
        assert_eq!(output, ff(input.to_string()));
    }
}
