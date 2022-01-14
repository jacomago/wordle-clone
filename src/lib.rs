use std::{
    fmt::Debug,
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

#[derive(PartialEq, Debug)]
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

#[cfg(test)]
mod tests {
    use crate::{guess_status, Status};

    #[test]
    fn test_guess_status() {
        assert_eq!(guess_status("a", "a"), vec![('a', Status::Correct)]);
        assert_eq!(guess_status("a", "b"), vec![('a', Status::Incorrect)]);
        assert_eq!(
            guess_status("ab", "ba"),
            vec![('a', Status::WrongPosition), ('b', Status::WrongPosition)]
        );

        assert_eq!(
            guess_status("word", "fred"),
            vec![
                ('w', Status::Incorrect),
                ('o', Status::Incorrect),
                ('r', Status::WrongPosition),
                ('d', Status::Correct)
            ]
        );
    }
}
