pub mod early_exit;
pub mod whole_len_check;
/// `#array&hashset`
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    early_exit::contains_duplicate(nums)
}
