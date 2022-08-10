pub fn check_inclusion(s1: String, s2: String) -> bool {
    let (mut counter_1, mut counter_2) = ([0; 26], [0; 26]);
    let l = s1.len();
    s1.as_bytes()
        .iter()
        .for_each(|&b| counter_1[(b - b'a') as usize] += 1);

    let sb_2 = s2.as_bytes();
    for (i, &b) in sb_2.iter().enumerate() {
        counter_2[(b - b'a') as usize] += 1;

        if i >= l {
            counter_2[(sb_2[i - l] - b'a') as usize] -= 1;
        }

        if i >= l - 1 && counter_1 == counter_2 {
            return true;
        }
    }
    false
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::permutation_in_string_567_::test::do_unit;

    #[test]
    fn unit() {
        do_unit("ab", "a", false, super::check_inclusion);
        do_unit("ab", "eidbaooo", true, super::check_inclusion);
        do_unit("ab", "eidboaoo", false, super::check_inclusion);
    }
}
