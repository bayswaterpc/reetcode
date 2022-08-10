use std::collections::HashMap;

pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }
    let mut s1_ltr_count: HashMap<char, usize> = HashMap::new();
    let mut s2_window: HashMap<char, usize> = HashMap::new();

    // Initializing window
    for ii in 0..s1.len() {
        let c = s1.chars().nth(ii).unwrap();
        let count = s1_ltr_count.entry(c).or_insert(0);
        *count += 1;

        let c = s2.chars().nth(ii).unwrap();
        let count = s2_window.entry(c).or_insert(0);
        *count += 1;
    }

    // Check Initial Window Criteria
    if s1_ltr_count == s2_window {
        return true;
    }

    for ii in s1.len()..s2.len() {
        // Remove left ltr update count & pop
        let c = s2.chars().nth(ii - s1.len()).unwrap();
        let count = s2_window.entry(c).or_insert(0);
        *count -= 1;
        if *count == 0 {
            s2_window.remove(&c);
        }
        // Add right ltr update count...
        let c = s2.chars().nth(ii).unwrap();
        let count = s2_window.entry(c).or_insert(0);
        *count += 1;

        // Check Window Criteria
        if s1_ltr_count == s2_window {
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
