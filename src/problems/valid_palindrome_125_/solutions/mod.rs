#[allow(unused)] //remove unused when implementing
pub fn is_palindrome(s: String) -> bool {
    let s_u8 = s.as_bytes();
    let mut ll = 0_i32;
    let mut rr = (s_u8.len() - 1) as i32;

    while ll < rr {
        while ll < rr && !(s_u8[ll as usize] as char).is_alphanumeric() {
            ll += 1;
        }
        while ll < rr && !(s_u8[rr as usize] as char).is_alphanumeric() {
            rr -= 1;
        }

        if (s_u8[rr as usize] as char).to_ascii_lowercase()
            != (s_u8[ll as usize] as char).to_ascii_lowercase()
        {
            return false;
        }
        ll += 1;
        rr -= 1;
    }

    true
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::valid_palindrome_125_::test::do_unit;

    #[test]
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit("a", true, super::is_palindrome);
        do_unit("A man, a plan, a canal: Panama", true, super::is_palindrome);
        do_unit("race a car", false, super::is_palindrome);
        do_unit(" ", true, super::is_palindrome);
    }
}
