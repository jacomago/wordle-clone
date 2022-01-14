use std::collections::{HashMap, HashSet};

use crate::{dictionary::pick_random_word, game::Game, Status};

fn word_check(word: &str, map: &WordMap) -> bool {
    let chars = word.chars().into_iter().collect::<Vec<char>>();
    for (c, pos) in &map.correct {
        if chars[*pos] != *c {
            return false;
        }
    }
    for (c, set) in &map.wrong_pos {
        if !chars.contains(c) {
            return false;
        }
        for pos in set {
            if chars[*pos] == *c {
                return false;
            }
        }
    }
    for c in chars {
        if map.incorrect.contains(&c) {
            return false;
        }
    }
    true
}

fn filter_set(set: HashSet<String>, map: &WordMap) -> HashSet<String> {
    set.into_iter()
        .filter(|word| word_check(word, map))
        .collect()
}

struct WordMap {
    correct: HashMap<char, usize>,
    wrong_pos: HashMap<char, HashSet<usize>>,
    incorrect: HashSet<char>,
}

impl WordMap {
    fn update_map(&mut self, word_status: &[(char, Status)]) {
        for (pos, (c, status)) in word_status.iter().enumerate() {
            match *status {
                Status::Incorrect => {
                    self.incorrect.insert(*c);
                }
                Status::Correct => {
                    self.correct.insert(*c, pos);
                }
                Status::WrongPosition => {
                    let entry = self.wrong_pos.entry(*c).or_insert_with(HashSet::new);
                    entry.insert(pos);
                }
            };
        }
    }
}

pub fn solve_game(game: &mut Game) -> String {
    let mut map = WordMap {
        correct: HashMap::new(),
        wrong_pos: HashMap::new(),
        incorrect: HashSet::new(),
    };
    let mut set = game.word_set.clone();
    loop {
        let guess = pick_random_word(&set);
        game.play_game(guess.to_string());

        if game.is_success() {
            return guess.to_string();
        }

        map.update_map(game.last_status().unwrap());
        set = filter_set(set, &map);
    }
}
