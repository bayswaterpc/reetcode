pub fn get_sum(a: i32, b: i32) -> i32 {
    let mut x = a.abs();
    let mut y = b.abs();
    //# ensure x >= y
    if x < y {
        return get_sum(b, a);
    }
    let sign = if a > 0 { 1 } else { -1 };

    if a * b >= 0 {
        //# sum of two positive integers
        while y != 0 {
            let tmp_x = x;
            x ^= y;
            y = (tmp_x & y) << 1;
        }
    } else {
        //# difference of two positive integers
        while y != 0 {
            let tmp_x = x;
            x ^= y;
            y = ((!tmp_x) & y) << 1;
        }
    }
    x * sign
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::sum_of_two_integers_371::test::do_unit;

    #[test]
    fn unit() {
        do_unit((1, 2), 3, super::get_sum);
        do_unit((2, 3), 5, super::get_sum);
        do_unit((-1, 4), 3, super::get_sum);
    }
}
