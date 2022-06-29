use std::borrow::Borrow;

#[derive(Default)]
pub struct Trie {
    children: [Option<Box<Trie>>; 26],
    end: bool,
}

#[derive(Default)]
pub struct WordDictionary {
    trie: Trie,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Adds a word into the data structure. */
    fn add_word(&mut self, word: String) {
        let mut node = &mut self.trie;
        for &c in word.as_bytes() {
            node = node.children[(c - b'a') as usize].get_or_insert(Box::new(Default::default()));
        }
        node.end = true;
    }

    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    fn search(&self, word: String) -> bool {
        WordDictionary::search_trie(&self.trie, word.as_bytes())
    }
    fn search_trie(trie: &Trie, word: &[u8]) -> bool {
        if let Some(&c) = word.first() {
            if c == b'.' {
                for child in trie.children.iter().flatten() {
                    if WordDictionary::search_trie(child.borrow(), &word[1..]) {
                        return true;
                    }
                }
            } else if let Some(node) = &trie.children[(c - b'a') as usize] {
                return WordDictionary::search_trie(node, &word[1..]);
            }
            false
        } else {
            trie.end
        }
    }
}

pub mod test {
    use super::WordDictionary;
    use crate::utils::test_utils::build_string_vec_from_str_line;

    pub fn test_word_dictionary(in_commands: &[&str], in_vals: &[&str], out_vals: &str) {
        if in_commands.is_empty()
            || in_vals.len() != in_commands.len()
            || in_commands[0] != "WordDictionary"
        {
            panic!("Invalid input")
        }

        let expected_outputs = build_string_vec_from_str_line(out_vals);
        if expected_outputs.len() != in_vals.len() {
            panic!("Invalid expected output")
        }
        let mut word_dictionary = WordDictionary::new();
        for (ii, command) in in_commands.iter().enumerate().skip(1) {
            match *command {
                "addWord" => {
                    word_dictionary.add_word(in_vals[ii].to_string());
                }
                "search" => {
                    println!("{}", expected_outputs[ii]);
                    let parsed_out: bool =
                        expected_outputs[ii].parse().expect("invalid boolean type");
                    assert_eq!(parsed_out, word_dictionary.search(in_vals[ii].to_string()));
                }
                _ => panic!("invalid input command"),
            }
        }
    }

    #[test]
    fn unit() {
        let in_commands = [
            "WordDictionary",
            "addWord",
            "addWord",
            "addWord",
            "search",
            "search",
            "search",
            "search",
        ];
        let in_vals = ["", "bad", "dad", "mad", "pad", "bad", ".ad", "b.."];
        let out_vals = "null,null,null,null,false,true,true,true";

        test_word_dictionary(&in_commands, &in_vals, &out_vals);
    }
}
