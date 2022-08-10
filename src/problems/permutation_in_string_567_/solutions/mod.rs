pub mod array_bytes;
pub mod hashmap_nth;

pub fn check_inclusion(s1: String, s2: String) -> bool {
    array_bytes::check_inclusion(s1, s2)
}
