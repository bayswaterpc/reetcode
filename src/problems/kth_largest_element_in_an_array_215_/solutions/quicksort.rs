pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    *nums.select_nth_unstable(n - k as usize).1
}

pub fn find_kth_largest_manual(nums: Vec<i32>, k: i32) -> i32 {
    fn q_sort(nums: &mut [i32]) {
        if nums.len() <= 1 {
            return;
        }

        let mid = nums.len() / 2;
        let (mut left, right) = (0, nums.len() - 1);
        nums.swap(mid, right);

        let mut i = 0;
        while i < nums.len() {
            if nums[i] < nums[right] {
                nums.swap(i, left);
                left += 1;
            }
            i += 1;
        }

        nums.swap(left, right);
        q_sort(&mut nums[0..left]);
        q_sort(&mut nums[left + 1..=right]);
    }

    let mut nums = nums;
    q_sort(&mut nums);
    nums[nums.len() - k as usize]
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::kth_largest_element_in_an_array_215_::test::do_unit;

    #[test]
    fn unit() {
        do_unit([3, 2, 1, 5, 6, 4], 2, 5, super::find_kth_largest);
        do_unit([3, 2, 3, 1, 2, 4, 5, 5, 6], 4, 4, super::find_kth_largest);

        do_unit([3, 2, 1, 5, 6, 4], 2, 5, super::find_kth_largest_manual);
        do_unit(
            [3, 2, 3, 1, 2, 4, 5, 5, 6],
            4,
            4,
            super::find_kth_largest_manual,
        );
    }
}
