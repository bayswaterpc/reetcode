use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn number_of_1_bits(n: u32) -> i32 {
        solutions::hammingWeight(n)
    }
}

pub mod test {
    #[allow(dead_code)]
    pub fn do_unit(input: &str, output: i32, hamming_weight: fn(u32) -> i32) {
        let n = u32::from_str_radix(input, 2).expect("Must be valid 32 bit int");
        assert_eq!(output, hamming_weight(n));
    }
}
