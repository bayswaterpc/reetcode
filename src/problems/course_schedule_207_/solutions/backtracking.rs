use std::collections::HashMap;

/// Solution to vec![Clone Graph](https://leetcode.com/problems/course-schedule/)
/// #time=O(E+V) where E is the number of courses and V is the number of prerequisites
/// #space=O(E+V)
// pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let num_courses = num_courses as usize;
    fn is_cyclic(
        pre2post: &HashMap<usize, Vec<usize>>,
        path: &mut Vec<bool>,
        visited_non_cyclic: &mut Vec<bool>,
        crs: usize,
    ) -> bool {
        if visited_non_cyclic[crs] {
            return false;
        }
        if path[crs] {
            return true;
        }
        path[crs] = true;

        if let Some(list) = pre2post.get(&crs) {
            for next_crs in list {
                if is_cyclic(pre2post, path, visited_non_cyclic, *next_crs) {
                    return true;
                }
            }
        }

        // backtrack on path
        path[crs] = false;
        visited_non_cyclic[crs] = true;
        false
    }

    let mut pre2post: HashMap<usize, Vec<usize>> = HashMap::new();
    for crs_pre in prerequisites {
        let (pre, crs) = (crs_pre[1] as usize, crs_pre[0] as usize);
        match pre2post.contains_key(&pre) {
            true => {
                pre2post.get_mut(&pre).unwrap().push(crs);
            }
            false => {
                pre2post.insert(pre, vec![crs]);
            }
        }
    }

    let mut path = vec![false; num_courses];
    let mut visited_non_cyclic = vec![false; num_courses];
    let range = 0..num_courses;
    let is_cyclic_val = range
        .into_iter()
        .any(|crs| is_cyclic(&pre2post, &mut path, &mut visited_non_cyclic, crs));

    !is_cyclic_val
}

mod test {
    #[test]
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
