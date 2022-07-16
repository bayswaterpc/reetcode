use crate::problems::encode_and_decode_strings_271_::EncodeDecode;

pub struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl EncodeDecode for Codec {
    fn new() -> Self {
        Codec {}
    }

    fn encode(&self, strs: Vec<String>) -> String {
        let mut res: String = String::from("");
        for s in strs {
            let chunk_header = std::char::from_u32(s.len() as u32).unwrap();
            res.push(chunk_header);
            res.push_str(&s);
        }
        res
    }

    fn decode(&self, s: String) -> Vec<String> {
        let chars: Vec<char> = s.chars().collect();

        let mut res = vec![];
        let mut cur = 0;
        while cur < chars.len() {
            let len = chars[cur] as usize;
            let chunk = chars[(cur + 1)..=(cur + len)].iter().collect();
            res.push(chunk);
            cur += 1 + len;
        }
        res
    }
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::encode_and_decode_strings_271_::test::do_unit;

    #[test] //replace allow with test when ready
            // Use VsCode run or debug options or..
            // change name to a unique test name and run `cargo test unq_name` or  run ..
            // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit::<2, super::Codec>(["Hello", "World"]);
        do_unit::<1, super::Codec>([""]);
    }
}
