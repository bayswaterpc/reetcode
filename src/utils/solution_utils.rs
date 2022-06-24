pub fn find_pivoted_start_index(nums: &[i32]) -> Option<usize> {
    // handles normally sorted, all the same, & len 1 cases
    if nums[0] <= *nums.last().unwrap() {
        return Some(0);
    }
    let (mut ll, mut rr) = (0, nums.len() - 1);

    while ll <= rr {
        let mid = (ll + rr) / 2;

        let b4mid = match mid {
            0 => nums.len() - 1,
            _ => mid - 1,
        };
        if nums[b4mid] > nums[mid] {
            return Some(mid);
        } else if nums[mid] < nums[0] {
            rr = mid - 1
        } else {
            ll = mid + 1
        }
    }
    None
}
