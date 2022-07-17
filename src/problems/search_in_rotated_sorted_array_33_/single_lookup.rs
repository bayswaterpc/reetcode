#[allow(dead_code)]
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut lo = 0;
    let mut hi = nums.len() - 1;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;

        if nums[mid] == target {
            return mid as i32;
        }

        if nums[lo] <= nums[mid] {
            if nums[lo] <= target && target < nums[mid] {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        } else if nums[mid] < nums[hi] {
            if nums[mid] < target && target <= nums[hi] {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
    }

    -1
}

mod test {
    #[allow(unused_imports)]
    use super::search;

    #[test]
    fn unit() {
        let nums = vec![3, 1];
        let target = 0;
        let output = -1;
        assert_eq!(output, search(nums, target));

        let nums = vec![1, 3];
        let target = 0;
        let output = -1;
        assert_eq!(output, search(nums, target));

        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        let output = 4;
        assert_eq!(output, search(nums, target));

        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        let output = -1;
        assert_eq!(output, search(nums, target));

        let nums = vec![1];
        let target = 0;
        let output = -1;
        assert_eq!(output, search(nums, target));
    }
}
