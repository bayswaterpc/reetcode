use super::Solution;

impl Solution {
    /// Solution to [Container With Most Water](https://leetcode.com/problems/container-with-most-water/)
    // in leetcode use below def
    //pub fn max_area(height: Vec<i32>) -> i32 {
    pub fn container_with_most_water(height: Vec<i32>) -> i32 {
        let mut ll = 0;
        let mut rr = height.len() - 1;
        let mut max_area = 0;

        while ll < rr {
            let new_area = (std::cmp::min(height[ll], height[rr])) * (rr - ll) as i32;
            max_area = std::cmp::max(max_area, new_area);

            if height[ll] < height[rr] {
                ll += 1;
            } else {
                rr -= 1;
            }
        }

        max_area
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let output = 49;
        assert_eq!(output, Solution::container_with_most_water(height));

        let height = vec![1, 1];
        let output = 1;
        assert_eq!(output, Solution::container_with_most_water(height));
    }
}
