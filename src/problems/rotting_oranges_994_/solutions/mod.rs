use std::collections::VecDeque;

pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut rotten = VecDeque::new();
    let (m, n) = (grid.len(), grid[0].len());

    let mut fresh_oranges = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == 2 {
                rotten.push_back((i, j));
            } else if *val == 1 {
                fresh_oranges += 1;
            }
        }
    }

    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut days = 0;

    while !rotten.is_empty() {
        let mut decayed = false;
        for _ in 0..rotten.len() {
            let (i, j) = rotten.pop_front().unwrap();

            for (dx, dy) in dirs.iter() {
                let x = i as i32 + dx;
                let y = j as i32 + dy;

                if x >= 0 && y >= 0 && x < m as i32 && y < n as i32 {
                    let (x, y) = (x as usize, y as usize);

                    if grid[x][y] == 1 {
                        grid[x][y] = 2;
                        fresh_oranges -= 1;
                        rotten.push_back((x, y));
                        decayed = true;
                    }
                }
            }
        }

        if decayed {
            days += 1;
        }
    }

    if fresh_oranges > 0 {
        -1
    } else {
        days
    }
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::rotting_oranges_994_::test::do_unit;

    #[test] //replace allow with test when ready
    fn unit() {
        do_unit([[2, 1, 1], [1, 1, 0], [0, 1, 1]], 4, super::oranges_rotting);
        do_unit(
            [[2, 1, 1], [0, 1, 1], [1, 0, 1]],
            -1,
            super::oranges_rotting,
        );
        do_unit([[0, 2]], 0, super::oranges_rotting);
    }
}
