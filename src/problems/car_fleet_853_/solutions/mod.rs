#[allow(unused)]
pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut pos_speed = position
        .into_iter()
        .zip(speed.into_iter())
        .collect::<Vec<_>>();

    // sort descending by position
    pos_speed.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    // ascending order stack of group arrival times
    let get_arrival_time =
        |pos: &i32, speed: &i32| -> f32 { (target - *pos) as f32 / *speed as f32 };
    let mut last_arrival = get_arrival_time(&pos_speed[0].0, &pos_speed[0].1);
    let mut fleet_count = 1;
    for (p, s) in pos_speed.iter().skip(1) {
        let new_arrival = get_arrival_time(p, s);
        if new_arrival > last_arrival {
            last_arrival = new_arrival;
            fleet_count += 1;
        }
    }
    fleet_count
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::car_fleet_853_::test::do_unit;

    #[test]
    fn unit() {
        do_unit(10, [6, 8], [3, 2], 2, super::car_fleet);
        do_unit(12, [10, 8, 0, 5, 3], [2, 4, 1, 1, 3], 3, super::car_fleet);
        do_unit(10, [3], [3], 1, super::car_fleet);
        do_unit(100, [0, 2, 4], [4, 2, 1], 1, super::car_fleet);
    }
}
