use std::collections::HashSet;
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let nums = nums.into_iter().collect::<HashSet<i32>>();
    let mut visited: HashSet<i32> = HashSet::new();
    let mut longest_streak = 0;
    for n in nums.iter() {
        let mut n = *n;
        if visited.contains(&n) {
            continue;
        }
        let mut current_streak = 1;
        while nums.contains(&(n + 1)) {
            n += 1;
            visited.insert(n);
            current_streak += 1;
            longest_streak = longest_streak.max(current_streak);
        }
    }
    longest_streak
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::longest_consecutive_sequence_128_::test::do_unit;

    #[test] //replace allow with test when ready
            // Use VsCode run or debug options or..
            // change name to a unique test name and run `cargo test unq_name` or  run ..
            // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit([100, 4, 200, 1, 3, 2], 4, super::longest_consecutive);
        do_unit(
            [0, 3, 7, 2, 5, 8, 4, 6, 0, 1],
            9,
            super::longest_consecutive,
        );
    }
}
