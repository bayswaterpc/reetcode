pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut left_limit_stack = vec![None];
    let mut max_area = 0;
    for ii in 0..heights.len() {
        let mut left_limit: Option<usize> = *left_limit_stack.last().unwrap();
        while left_limit.is_some() && heights[ii] <= heights[left_limit.unwrap()] {
            let curr_height = heights[left_limit_stack.pop().unwrap().unwrap()];
            let curr_width = if left_limit_stack.last().unwrap().is_some() {
                (ii - left_limit_stack.last().unwrap().unwrap() - 1) as i32
            } else {
                (ii + 1 - 1) as i32
            };
            max_area = max_area.max(curr_height * curr_width);
            left_limit = *left_limit_stack.last().unwrap();
        }
        left_limit_stack.push(Some(ii));
    }

    let mut left_limit: Option<usize> = *left_limit_stack.last().unwrap();
    while left_limit.is_some() {
        let curr_height = heights[left_limit_stack.pop().unwrap().unwrap()];
        let curr_width = if left_limit_stack.last().unwrap().is_some() {
            (heights.len() - left_limit_stack.last().unwrap().unwrap() - 1) as i32
        } else {
            (heights.len()) as i32
        };
        max_area = max_area.max(curr_height * curr_width);
        left_limit = *left_limit_stack.last().unwrap();
    }

    max_area
}

pub mod test {
    #[allow(unused)]
    use super::super::test::do_unit;

    #[test]
    fn unit() {
        do_unit([2, 1, 5, 6, 2, 3], 10, super::largest_rectangle_area);
        do_unit([2, 4], 4, super::largest_rectangle_area);
    }
}
