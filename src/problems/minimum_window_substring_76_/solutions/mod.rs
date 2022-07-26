const MAP_SIZE: usize = 256;

pub fn min_window(s: String, t: String) -> String {
    use std::collections::VecDeque;
    let tmap = get_char_set(&t[..]);
    // number of unique characters in t
    let tcount: usize = tmap.iter().filter(|x| x != &&0).count();
    // number of unique characters of t seen
    let mut tseen = 0;
    // current best candidate
    let mut candidate: Option<(usize, usize)> = None;
    // current left bound
    let mut l = 0usize;
    // current right bound
    let mut r = 0usize;
    // next left bound
    let mut q: VecDeque<usize> = VecDeque::new();
    // count of characters seen
    let mut seen = [0usize; MAP_SIZE];
    // byte representation of string since we're dealing only with ASCII
    let raw = s.as_bytes();
    while l < raw.len() && tmap[raw[l] as usize] < 1 {
        l += 1;
    }
    // none of the chars were in l
    if l >= raw.len() {
        return "".to_string();
    }
    q.push_back(l);
    while !q.is_empty() {
        let next_l = q.pop_front().unwrap();
        // shrink current window
        while l < next_l {
            if seen[raw[l] as usize] > 0 {
                seen[raw[l] as usize] -= 1;
                if seen[raw[l] as usize] < tmap[raw[l] as usize] {
                    tseen -= 1;
                }
            }
            l += 1;
        }
        // check if we have a solution
        if tseen == tcount {
            if candidate.is_none() && r >= l {
                candidate = Some((l, r));
            } else if let Some((lb, rb)) = candidate {
                if rb - lb > r - l {
                    candidate = Some((l, r));
                }
            }
        }
        // only needed for r = 0
        if r < l {
            r = l;
        }
        // try expanding window
        while r < raw.len() && tseen < tcount {
            seen[raw[r] as usize] += 1;
            if tmap[raw[r] as usize] > 0 {
                q.push_back(r);
                // println!("{} {} {}", raw[r] as char, seen[raw[r] as usize], tmap[raw[r] as usize]);
                if seen[raw[r] as usize] == tmap[raw[r] as usize] {
                    tseen += 1;
                }
            }
            r += 1;
            // println!("Substring considered: {} -> tseen: {}", &s[l..r], tseen);
        }
        // check if we have a solution
        if tseen == tcount {
            if candidate.is_none() {
                candidate = Some((l, r));
            } else if let Some((lb, rb)) = candidate {
                if rb - lb > r - l {
                    candidate = Some((l, r));
                }
            }
        } else {
            // break because we don't have a solution till the end of the string
            break;
        }
    }
    if let Some((lb, rb)) = candidate {
        s[lb..rb].to_string()
    } else {
        "".to_string()
    }
}

fn get_char_set(t: &str) -> [usize; MAP_SIZE] {
    let mut result = [0; MAP_SIZE];
    t.as_bytes().iter().for_each(|c| {
        result[*c as usize] += 1;
    });
    result
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::minimum_window_substring_76_::test::do_unit;

    #[test] //replace allow with test when ready
            // Use VsCode run or debug options or..
            // change name to a unique test name and run `cargo test unq_name` or  run ..
            // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit(("ADOBECODEBANC", "ABC"), "BANC", super::min_window);
    }
}
