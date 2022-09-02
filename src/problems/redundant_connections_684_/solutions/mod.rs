use std::collections::HashSet;

fn are_connected(graph: &Vec<Vec<i32>>, seen: &mut HashSet<i32>, val: i32, target: i32) -> bool {
    if !seen.contains(&val) {
        seen.insert(val);
        if val == target {
            return true;
        }

        let index: usize = val as usize;
        for i in 0..graph[index].len() {
            if are_connected(graph, seen, graph[index][i], target) {
                return true;
            }
        }
    }
    false
}

pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut graph = vec![Vec::new(); 1001];
    let mut res = vec![0; 2];

    for i in 0..edges.len() {
        let mut seen = HashSet::new();
        if i != 0 && are_connected(&graph, &mut seen, edges[i][0], edges[i][1]) {
            res[0] = edges[i][0];
            res[1] = edges[i][1];
        }

        graph[edges[i][0] as usize].push(edges[i][1]);
        graph[edges[i][1] as usize].push(edges[i][0]);
    }

    res
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::redundant_connections_684_::test::do_unit;

    #[test]
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit(
            [[1, 2], [1, 3], [2, 3]],
            [2, 3],
            super::find_redundant_connection,
        );
        do_unit(
            [[1, 2], [2, 3], [3, 4], [1, 4], [1, 5]],
            [1, 4],
            super::find_redundant_connection,
        );
    }
}
