const SIZE: usize = 26; // Our tries contain only characters between a and z, both included.
const ASCII_OFFSET: usize = 97; // a's ASCII code.

/** Convert an ASCII char into an usize, such that 'a' -> 0, 'b' -> 1, ..., 'z' -> 25. */
fn to_index(c: char) -> usize {
    (c as usize) - ASCII_OFFSET
}

/// Solution to [Implement Trie (Prefix Tree)](https://leetcode.com/problems/implement-trie-prefix-tree/)
#[derive(Default)]
pub struct Trie {
    children: [Option<Box<Trie>>; SIZE], // We use Box as Trie is a recursive data type. See also: https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes
    end_of_word: bool,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }
    
    /// Inserts a word into the trie.
    pub fn insert(&mut self, word: String) {
        let mut node = self;
        for i in word.chars().map(to_index) {
            node = node.children[i].get_or_insert(Box::new(Trie::new()));
        }
        node.end_of_word = true;
    }
    
    fn node_search(&self, word: &String) -> Option<&Trie> {
        let mut node = self;
        for i in word.chars().map(to_index) {
            match node.children[i] {
                None => { return None; }, // Abort the search as soon as possible.
                Some(ref child) => { node = child; },
            }
        }
        Some(node)
    }
    
    /// Checks if the word is in the trie
    fn search(&self, word: String) -> bool {
        Self::node_search(self, &word).map(|node| node.end_of_word).unwrap_or(false)
    }
    
    /// Checks if the prefix is in the trie
    fn starts_with(&self, prefix: String) -> bool {
        Self::node_search(self, &prefix).is_some()
    }
}

pub mod test {
    use super::Trie;
    use crate::utils::test_utils::build_string_vec_from_str_line;

    pub fn test_trie_commands(in_commands: &[&str], in_vals: &[&str], out_vals: &str) {
        if in_commands.is_empty() || in_vals.len() != in_commands.len()|| in_commands[0] != "Trie" {
            panic!("Invalid input")
        }

        let expected_outputs = build_string_vec_from_str_line(out_vals);
        if expected_outputs.len() != in_vals.len(){
            panic!("Invalid expected output")
        }
        let mut trie = Trie::new();
        for (ii, command) in in_commands.iter().enumerate().skip(1) {
            match *command {
                "insert" => {
                    trie.insert(in_vals[ii].to_string());
                },
                "search" => {
                    println!("{}", expected_outputs[ii]);
                    let parsed_out: bool = expected_outputs[ii].parse().expect("invalid boolean type");
                    assert_eq!(parsed_out, trie.search(in_vals[ii].to_string()));
                },
                "startsWith" => {
                    let parsed_out: bool = expected_outputs[ii].parse().expect("invalid boolean type");
                    assert_eq!(parsed_out, trie.starts_with(in_vals[ii].to_string()));
                },
                _ => panic!("invalid input command"),
            }
        }
    }

    #[test]
    fn unit() {
        let in_commands = ["Trie", "insert", "search", "search", "startsWith", "insert", "search"];
        let in_vals = ["", "apple", "apple", "app", "app", "app", "app"];
        let out_vals = "null, null, true, false, true, null, true";

        test_trie_commands(&in_commands, &in_vals, &out_vals);
    }
}
