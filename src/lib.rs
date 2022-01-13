use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use std::collections::HashSet;

pub mod dictionary;
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub enum Status {
    Incorrect,
    Correct,
    WrongPosition,
}

pub fn invalid_guess(guess: &str, word_length: usize, set: &HashSet<String>) -> bool {
    if guess.len() != word_length {
        println!("Guess must be of correct length.");
        false
    } else if !set.contains(guess) {
        println!("Guess not in dictionary.");
        false
    } else {
        true
    }
}

pub fn is_success(statuses: &Vec<Status>) -> bool {
    for status in statuses {
        if match status.to_owned() {
            Status::Incorrect => true,
            Status::WrongPosition => true,
            Status::Correct => false,
        } {
            return false;
        }
    }
    true
}

pub fn guess_status(guess: &str, word: &str) -> Vec<Status> {
    let word_chars = word.chars().collect::<Vec<char>>();

    let mut statuses = vec![];
    let mut count = 0;
    for c in guess.chars() {
        let status = if c == word_chars.get(count).unwrap().to_owned() {
            Status::Correct
        } else if word_chars.contains(&c) {
            Status::WrongPosition
        } else {
            Status::Incorrect
        };
        statuses.push(status);

        count += 1;
    }
    statuses
}
