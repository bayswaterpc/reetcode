pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    if piles.len() > h as usize {
        panic!("h must be greater than number of piles");
    }
    let mut l = 1;
    let mut r = *piles.iter().max().unwrap();
    while l < r {
        let mid = (l + r) / 2;
        //#![feature(int_roundings)]
        if piles.iter().map(|p| (*p + mid - 1) / mid).sum::<i32>() > h {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    r
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::koko_eating_bananas_875_::test::do_unit;

    #[test]
    fn unit() {
        do_unit([312884470], 968709470, 1, super::min_eating_speed);
        do_unit([3,6,7,11], 8, 4, super::min_eating_speed);
        do_unit([30,11,23,4,20], 5, 30, super::min_eating_speed);
        do_unit([30,11,23,4,20], 6, 23, super::min_eating_speed);
    }
}
