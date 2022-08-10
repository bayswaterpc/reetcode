use std::cmp::Ordering;

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut l = 0_usize;
    let mut r = numbers.len() - 1_usize;

    while l < r {
        let sum = numbers[l] + numbers[r];
        match sum.cmp(&target) {
            Ordering::Equal => return vec![l as i32 + 1, r as i32 + 1],
            Ordering::Less => l += 1,
            Ordering::Greater => r -= 1,
        }
    }
    panic!("No two sum solution");
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::two_sum_ii_167_::test::do_unit;

    #[test]
    fn unit() {
        do_unit([2,7,11,15], 9, [1,2], super::two_sum);
        do_unit([2,3,4], 6, [1,3], super::two_sum);
        do_unit([-1,0], -1, [1,2], super::two_sum);
    }
}
