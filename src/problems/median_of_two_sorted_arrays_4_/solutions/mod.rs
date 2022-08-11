pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let first;
    let second;
    if nums1.len() <= nums2.len() {
        first = nums1;
        second = nums2;
    } else {
        first = nums2;
        second = nums1;
    }

    let mut low = 0;
    let mut high = first.len();

    let mut part_x;
    let mut part_y;

    while low <= high {
        part_x = (low + high) / 2;
        part_y = (first.len() + second.len() + 1) / 2 - part_x;

        let one_left = if part_x == 0 {
            std::i32::MIN
        } else {
            first[part_x - 1]
        };
        let one_right = if part_x == first.len() {
            std::i32::MAX
        } else {
            first[part_x]
        };
        let two_left = if part_y == 0 {
            std::i32::MIN
        } else {
            second[part_y - 1]
        };
        let two_right = if part_y == second.len() {
            std::i32::MAX
        } else {
            second[part_y]
        };

        if one_left <= two_right && two_left <= one_right {
            if (first.len() + second.len()) % 2 == 0 {
                return f64::from(one_left.max(two_left) + one_right.min(two_right)) / 2.0;
            } else {
                return f64::from(one_left.max(two_left));
            }
        } else if one_left > two_right {
            high = part_x - 1;
        } else {
            low = part_x + 1;
        }
    }
    panic!("Wrong arrays");
}
pub mod test {
    #[allow(unused_imports)]
    use crate::problems::median_of_two_sorted_arrays_4_::test::do_unit;

    #[test]
    fn unit() {
        do_unit([1, 2], [2], 2.0, super::find_median_sorted_arrays);
        do_unit([1, 2], [3, 4], 2.5, super::find_median_sorted_arrays);
    }
}
