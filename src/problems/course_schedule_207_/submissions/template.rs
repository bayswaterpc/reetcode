#[allow(unused)] // remove allow when implementing
pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    false
}

mod test {

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name`
    fn unit() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        let output = true;
        assert_eq!(output, super::can_finish(num_courses, prerequisites));

        let num_courses = 2;
        let prerequisites = vec![vec![1, 0], vec![0, 1]];
        let output = false;
        assert_eq!(output, super::can_finish(num_courses, prerequisites));

        let num_courses = 3;
        let prerequisites = vec![vec![1, 0], vec![0, 2], vec![2, 1]];
        let output = false;
        assert_eq!(output, super::can_finish(num_courses, prerequisites));
    }
}
