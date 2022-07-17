use super::Solution;
mod single_lookup;
use crate::utils::solution_utils::find_pivoted_start_index;

// #space=O(1)
// #time=O(log(N))

// Returns index if found else -1
fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
    let (mut ll, mut rr) = (0, nums.len() - 1);

    while ll <= rr {
        let mid = (ll + rr) / 2;

        match nums[mid].cmp(&target) {
            std::cmp::Ordering::Less => ll = mid + 1,
            std::cmp::Ordering::Equal => {
                return Some(mid);
            }
            std::cmp::Ordering::Greater => {
                if mid == 0 {
                    return None;
                }
                rr = mid - 1
            }
        }
    }
    None
}

impl Solution {
    /// Solution to [Search in Rotated Sorted Array](https://leetcode.com/problems/search-in-rotated-sorted-array/)
    // in leetcode use below def
    // pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    pub fn search_in_rotated_sorted_array(nums: Vec<i32>, target: i32) -> i32 {
        if nums[0] < *nums.last().unwrap() {
            return match binary_search(nums.as_slice(), target) {
                Some(index) => index as i32,
                None => -1,
            };
        } else if nums.len() == 1 {
            return match nums[0].eq(&target) {
                true => 0,
                false => -1,
            };
        }
        let start = find_pivoted_start_index(nums.as_slice()).unwrap();

        if target >= nums[start] && target <= *nums.last().unwrap() {
            match binary_search(&nums[start..], target) {
                Some(index) => (start + index) as i32,
                None => -1,
            }
        } else if target >= nums[0] && target <= nums[start - 1] {
            match binary_search(&nums[..start], target) {
                Some(index) => index as i32,
                None => -1,
            }
        } else {
            -1
        }
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let nums = vec![3, 1];
        let target = 0;
        let output = -1;
        assert_eq!(
            output,
            Solution::search_in_rotated_sorted_array(nums, target)
        );

        let nums = vec![1, 3];
        let target = 0;
        let output = -1;
        assert_eq!(
            output,
            Solution::search_in_rotated_sorted_array(nums, target)
        );

        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        let output = 4;
        assert_eq!(
            output,
            Solution::search_in_rotated_sorted_array(nums, target)
        );

        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        let output = -1;
        assert_eq!(
            output,
            Solution::search_in_rotated_sorted_array(nums, target)
        );

        let nums = vec![1];
        let target = 0;
        let output = -1;
        assert_eq!(
            output,
            Solution::search_in_rotated_sorted_array(nums, target)
        );
    }
}
