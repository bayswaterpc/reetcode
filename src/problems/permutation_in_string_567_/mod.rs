use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        solutions::check_inclusion(s1, s2)
    }
}

pub mod test {
    #[allow(dead_code)]
    pub fn do_unit(
        s1: &str,
        s2: &str,
        output: bool,
        check_inclusion: fn(s1: String, s2: String) -> bool,
    ) {
        assert_eq!(output, check_inclusion(s1.to_string(), s2.to_string()));
    }
}
