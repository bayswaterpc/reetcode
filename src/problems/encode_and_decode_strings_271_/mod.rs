use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn encode_and_decode_strings_271() -> solutions::Codec {
        solutions::Codec::new()
    }
}

pub trait EncodeDecode {
    fn new() -> Self;
    fn encode(&self, strs: Vec<String>) -> String;
    fn decode(&self, s: String) -> Vec<String>;
}

pub mod test {
    #[allow(dead_code)]
    pub fn do_unit<const N: usize, T: super::EncodeDecode>(input: [&str; N]) {
        let input: Vec<String> = input.into_iter().map(|s| s.to_string()).collect();
        let encoder_decoder = T::new();
        let encoded = encoder_decoder.encode(input.clone());
        let decoded = encoder_decoder.decode(encoded);
        assert_eq!(input, decoded);
    }
}
