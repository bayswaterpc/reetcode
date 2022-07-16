use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn minimum_window_substring_76_(s: String, t: String) -> String {
        solutions::min_window(s, t)
    }
}

pub mod test {
    #[allow(dead_code)]
    pub fn do_unit(input: (&str, &str), output: &str, min_window: fn(String, String) -> String) {
        assert_eq!(
            output.to_string(),
            min_window(input.0.to_string(), input.1.to_string())
        );
    }
}
