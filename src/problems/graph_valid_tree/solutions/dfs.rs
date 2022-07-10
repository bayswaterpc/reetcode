use std::collections::HashSet;

/// time=O(N)
/// space=O(N)
pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
    let n = n as usize;
    if edges.len() != n-1 {
        return false;
    }

    let mut adj_list: Vec<Vec<usize>> = vec![vec![]; n];
    for edge in edges.into_iter() {
        let (node, nbr) = (edge[0] as usize, edge[1] as usize);
        adj_list[node].push(nbr);
        adj_list[nbr].push(node);
    }

    let mut seen: HashSet<usize> = HashSet::from([0]);
    let mut stack = vec![0];
    while !stack.is_empty() {
        let node = stack.pop().unwrap();
        for nbr in adj_list[node].iter() {
            if seen.contains(nbr) {
                continue;
            }
            seen.insert(*nbr);
            stack.push(*nbr);
        }
    }
    seen.len() == n
}


pub mod test {
    #[allow(unused_imports)]
    use crate::utils::test_utils::array2d_to_vec2d;

    #[test]
    fn unit() {
        let n = 5;
        let edges = array2d_to_vec2d([[0,1],[0,2],[0,3],[1,4]]);
        let output = true;
        assert_eq!(output, super::valid_tree(n, edges));

        let n = 5;
        let edges = array2d_to_vec2d([[0,1],[1,2],[2,3],[1,3],[1,4]]);
        let output = false;
        assert_eq!(output, super::valid_tree(n, edges));
    }
}
