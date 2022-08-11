use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn median_of_two_sorted_arrays_4_(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        solutions::find_median_sorted_arrays(nums1, nums2)
    }
}

pub mod test {
    pub fn do_unit<
        const M: usize,
        const N: usize,
        T: Clone + PartialEq + std::fmt::Debug + Sized,
        R: Clone + PartialEq + std::fmt::Debug + Sized,
    >(
        n1: [T; N],
        n2: [T; M],
        output: R,
        find_median_sorted_arrays: fn(Vec<T>, Vec<T>) -> R,
    ) {
        assert_eq!(output, find_median_sorted_arrays(n1.to_vec(), n2.to_vec()));
    }
}
