use super::Solution;
use std::collections::{HashMap, HashSet};

fn construct_undirected_graph(edges: Vec<Vec<i32>>) -> HashMap<usize, Vec<usize>> {
    let mut undirected_graph: HashMap<usize, Vec<usize>> = HashMap::new();

    for edge in edges.into_iter() {
        let (head, tail) = (edge[0] as usize, edge[1] as usize);
        match undirected_graph.contains_key(&head) {
            true => {
                undirected_graph.get_mut(&head).unwrap().push(tail);
            }
            false => {
                undirected_graph.insert(head, vec![tail]);
            }
        }
        match undirected_graph.contains_key(&tail) {
            true => {
                undirected_graph.get_mut(&tail).unwrap().push(head);
            }
            false => {
                undirected_graph.insert(tail, vec![head]);
            }
        }
    }
    undirected_graph
}

impl Solution {
    /// Solution to [Number of Connected Components in an Undirected Graph](https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/submissions/)
    /// #time=O(E+V) where E is the number of courses and V is the number of prerequisites
    /// #space=O(E+V)
    //pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    pub fn number_of_connected_components_in_an_undirected_graph(
        n: i32,
        edges: Vec<Vec<i32>>,
    ) -> i32 {
        let undirected_graph = construct_undirected_graph(edges);
        let mut visited: HashSet<usize> = HashSet::new();
        fn dfs(
            undirected_graph: &HashMap<usize, Vec<usize>>,
            node: &usize,
            visited: &mut HashSet<usize>,
        ) {
            if visited.contains(node) {
                return;
            }
            visited.insert(*node);

            if let Some(list) = undirected_graph.get(node) {
                for nbr in list.iter() {
                    dfs(undirected_graph, nbr, visited);
                }
            }
        }

        let mut num_components = 0;
        for node in 0..(n as usize) {
            if visited.contains(&node) {
                continue;
            }
            num_components += 1;
            dfs(&undirected_graph, &node, &mut visited);
        }
        num_components
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let n = 5;
        let edges = vec![vec![0, 1], vec![1, 2], vec![3, 4]];
        let output = 2;
        assert_eq!(
            output,
            Solution::number_of_connected_components_in_an_undirected_graph(n, edges)
        );

        let n = 5;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
        let output = 1;
        assert_eq!(
            output,
            Solution::number_of_connected_components_in_an_undirected_graph(n, edges)
        );

        let n = 5;
        let edges = vec![vec![0, 1], vec![1, 2], vec![0, 2], vec![3, 4]];
        let output = 2;
        assert_eq!(
            output,
            Solution::number_of_connected_components_in_an_undirected_graph(n, edges)
        );
    }
}
