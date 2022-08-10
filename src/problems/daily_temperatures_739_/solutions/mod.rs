#[allow(unused)] //remove unused when implementing
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<usize> = Vec::new();
    let mut ans = vec![0; temperatures.len()];

    for ii in 0..temperatures.len() {
        let t = temperatures[ii];
        while !stack.is_empty() && temperatures[*stack.last().unwrap()] < t {
            let colder_day = stack.pop().unwrap();
            ans[colder_day] = ii as i32 - colder_day as i32;
        }
        stack.push(ii);
    }

    ans
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::daily_temperatures_739_::test::do_unit;

    #[test]
    fn unit() {
        do_unit(
            [73, 74, 75, 71, 69, 72, 76, 73],
            [1, 1, 4, 2, 1, 1, 0, 0],
            super::daily_temperatures,
        );
        do_unit([30, 40, 50, 60], [1, 1, 1, 0], super::daily_temperatures);
        do_unit([30, 60, 90], [1, 1, 0], super::daily_temperatures);
    }
}
