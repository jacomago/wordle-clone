use std::collections::{HashMap, HashSet, hash_map::Keys};

use rand::{prelude::IteratorRandom, thread_rng};

use crate::read_lines;

pub struct Dictionary(HashMap<usize, HashSet<String>>);

pub fn load_dictionary(filename: &str) -> Dictionary {
    let mut output = HashMap::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for word in lines.flatten() {
            let word_length = word.len();
            let set = output
                .entry(word_length as usize)
                .or_insert_with(HashSet::new);
            set.insert(word.to_lowercase());
        }
    } else {
        panic!("no lines");
    }
    Dictionary(output)
}

pub fn pick_random_word(set: &HashSet<String>) -> &str {
    let mut rng = thread_rng();
    set.iter().choose(&mut rng).unwrap()
}

impl Dictionary {
    pub fn gen_word(&self, word_length: usize) -> &str {
        let set = self
            .get_set(word_length)
            .expect("Expected set of such words.");
        pick_random_word(set)
    }

    pub fn get_set(&self, word_length: usize) -> Option<&HashSet<String>> {
        self.0.get(&word_length)
    }

    pub fn keys(&self) -> Keys<'_, usize, std::collections::HashSet<std::string::String>, > {
        self.0.keys()
    }
}
