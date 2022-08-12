use std::cmp;
use std::collections::HashMap;

pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    let mut tasks_count: HashMap<char, i32> = HashMap::new();
    for i in tasks.iter() {
        tasks_count
            .entry(*i)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let max_count: i32 = *tasks_count.values().max().unwrap();
    let max_count_tasks: i32 = tasks_count.values().filter(|&x| *x == max_count).count() as i32;
    cmp::max(
        tasks.len() as i32,
        (max_count - 1) * (n + 1) + max_count_tasks,
    )
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::task_scheduler_621_::test::do_unit;

    #[test]
    fn unit() {
        do_unit(['A', 'A', 'A', 'B', 'B', 'B'], 2, 8, super::least_interval);
        do_unit(['A', 'A', 'A', 'B', 'B', 'B'], 0, 6, super::least_interval);
        do_unit(
            ['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
            2,
            16,
            super::least_interval,
        );
    }
}
