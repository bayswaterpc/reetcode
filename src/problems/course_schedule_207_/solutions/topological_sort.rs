use std::collections::{HashMap, VecDeque};

#[derive(Default, Debug, Clone)]
struct DagNode {
    in_degrees: usize,
    out_nodes: Vec<i32>,
}

pub fn can_finish(_num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut graph: HashMap<i32, DagNode> = HashMap::new();

    let total_deps = prerequisites.len();
    // Build the Dag
    for relation in prerequisites.into_iter() {
        let (next, pre) = (relation[1], relation[0]);
        graph.entry(pre).or_default().out_nodes.push(next);
        graph.entry(next).or_default().in_degrees += 1;
    }
    println!("{:?}", graph);

    // Build the queue of no dependency courses
    let mut no_dependency_queue: VecDeque<i32> = VecDeque::new();
    for (crs_key, node) in graph.iter() {
        if node.in_degrees == 0 {
            no_dependency_queue.push_back(*crs_key);
        }
    }
    println!("{:?}", no_dependency_queue);

    let mut removed_edges = 0;
    while !no_dependency_queue.is_empty() {
        let crs_key = no_dependency_queue.pop_front().unwrap();
        let next_nodes = graph.get(&crs_key).unwrap().out_nodes.clone();
        for next in next_nodes.iter() {
            graph.entry(*next).or_default().in_degrees -= 1;
            removed_edges += 1;
            if graph.get(next).unwrap().in_degrees == 0 {
                no_dependency_queue.push_back(*next);
            }
        }
    }

    removed_edges == total_deps
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
