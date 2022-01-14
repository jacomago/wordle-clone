use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub mod dictionary;
pub mod game;

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

pub fn guess_status(guess: &str, word: &str) -> Vec<(char, Status)> {
    let word_chars = word.chars().collect::<Vec<char>>();

    let mut statuses = vec![];
    for (count, c) in guess.chars().enumerate() {
        let status = if c == *word_chars.get(count).unwrap() {
            Status::Correct
        } else if word_chars.contains(&c) {
            Status::WrongPosition
        } else {
            Status::Incorrect
        };
        statuses.push((c, status));

    }
    statuses
}
