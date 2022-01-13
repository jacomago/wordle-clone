use std::collections::{HashMap, HashSet};

use rand::{prelude::IteratorRandom, thread_rng};

use crate::read_lines;

pub struct Dictionary(HashMap<usize, HashSet<String>>);

pub fn load_dictionary(filename: &str) -> Dictionary {
    let mut output = HashMap::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(word) = line {
                let word_length = word.len();
                let set = output.entry(word_length as usize).or_insert(HashSet::new());
                set.insert(word.to_lowercase());
            }
        }
    } else {
        panic!("no lines");
    }
    Dictionary(output)
}

impl Dictionary {
    pub fn gen_word(&self, word_length: usize) -> &str {
        let mut rng = thread_rng();
        let set = self
            .0
            .get(&word_length)
            .expect("Expected set of such words.");
        set.into_iter().choose(&mut rng).unwrap()
    }

    pub fn get_set(&self, word_length: usize) -> Option<&HashSet<String>> {
        self.0.get(&word_length)
    }
}
