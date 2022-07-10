struct UnionFind {
    parents: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let parents = (0..n).map(|n| n).collect();
        UnionFind { parents }
    }

    /// Finds parent of node
    pub fn find(&self, node: &usize) -> usize {
        let mut parent = *node;
        while parent != self.parents[parent] {
            parent = self.parents[parent]
        }
        parent
    }

    /// Return true if able to merge, false if already in the same set
    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let root_a = self.find(&a);
        let root_b = self.find(&b);

        // Check if
        if root_a == root_b {
            return false;
        }

        self.parents[root_a] = root_b;

        true
    }
}

#[allow(unused)] // remove allow when implementing
pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
    let n = n as usize;
    if edges.len() != n - 1 {
        return false;
    }

    let mut union_find = UnionFind::new(n);

    !edges.into_iter().any(|edge| {
        let (a, b) = (edge[0] as usize, edge[1] as usize);
        !union_find.union(a, b)
    })

    //Equivalent imperitive approach
    // for edge in edges.iter() {
    //     let (a, b) = (edge[0] as usize, edge[1] as usize);
    //     if ! union_find.union(a, b){
    //         return false;
    //     }
    // }
    // true
}

pub mod test {
    #[allow(unused_imports)]
    use crate::utils::test_utils::array2d_to_vec2d;

    #[test]
    fn unit() {
        let n = 5;
        let edges = array2d_to_vec2d([[0, 1], [0, 2], [0, 3], [1, 4]]);
        let output = true;
        assert_eq!(output, super::valid_tree(n, edges));

        let n = 5;
        let edges = array2d_to_vec2d([[0, 1], [1, 2], [2, 3], [1, 3], [1, 4]]);
        let output = false;
        assert_eq!(output, super::valid_tree(n, edges));
    }
}
