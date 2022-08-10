pub fn trap(height: Vec<i32>) -> i32 {
    let h = height;
    let mut l = 0;
    let mut r = h.len() - 1;
    let mut res = 0;
    let mut lmax = 0;
    let mut rmax = 0;
    while l < r {
        if h[l] < h[r] {
            lmax = lmax.max(h[l]);
            res += lmax - h[l];
            l += 1;
        } else {
            rmax = rmax.max(h[r]);
            res += rmax - h[r];
            r -= 1;
        }
    }
    res
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::trapping_rain_water_42_::test::do_unit;

    #[test]
    fn unit() {
        do_unit([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6, super::trap);
        do_unit([4, 2, 0, 3, 2, 5], 9, super::trap);
    }
}
