use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn reverse_bits(n: u32) -> u32 {
        solutions::reverse_bits(n)
    }
}

pub mod test {
    #[allow(dead_code)]
    pub fn do_unit(input: &str, output: u32, reverse_bits: fn(u32) -> u32) {
        let n = u32::from_str_radix(input, 2).expect("Must be valid 32 bit u32 string");
        assert_eq!(output, reverse_bits(n));
    }
}
