use std::collections::HashSet;

pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let mut seen = HashSet::<(i32, i32)>::new();
    fn island_area(ii: i32, jj: i32, seen: &mut HashSet<(i32, i32)>, grid: &Vec<Vec<i32>>) -> i32 {
        if ii < 0
            || jj < 0
            || ii >= grid.len() as i32
            || jj >= grid[0].len() as i32
            || seen.contains(&(ii, jj))
            || grid[ii as usize][jj as usize] == 0
        {
            return 0;
        }
        seen.insert((ii, jj));
        let mut area = 1;
        for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
            let (ni, nj) = (ii + di, jj + dj);
            area += island_area(ni, nj, seen, grid);
        }
        area
    }
    let mut max_area = 0;
    for ii in 0..grid.len() {
        for jj in 0..grid[0].len() {
            max_area = max_area.max(island_area(ii as i32, jj as i32, &mut seen, &grid));
        }
    }
    max_area
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::max_area_of_island_695_::test::do_unit;

    #[test]
    fn unit() {
        do_unit(
            [
                [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                [0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                [0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
            ],
            6,
            super::max_area_of_island,
        );

        do_unit([[0, 0, 0, 0, 0, 0, 0, 0]], 0, super::max_area_of_island);
        do_unit([[0, 1, 1], [1, 1, 0]], 4, super::max_area_of_island);
    }
}
