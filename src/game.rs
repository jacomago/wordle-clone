use colored::Colorize;
use std::fmt::Write;
use std::{collections::HashSet, fmt};

use crate::{dictionary::Dictionary, guess_status, Status};

fn init_game(word_length: usize, dictionary: &Dictionary) -> GameState {
    let word = dictionary.gen_word(word_length).to_owned();

    GameState {
        correct_word: word,
        guesses: vec![],
    }
}

pub struct Game {
    pub word_set: HashSet<String>,
    word_length: usize,
    state: GameState,
}

impl Game {
    pub fn new(word_length: usize, dictionary: Dictionary) -> Game {
        let state = init_game(word_length, &dictionary);
        let word_set = dictionary.get_set(word_length).unwrap().to_owned();
        Game {
            word_set,
            word_length,
            state,
        }
    }

    pub fn play_game(&mut self, guess: String) {
        let status = guess_status(&guess, self.state.get_correct_word());

        self.state.update_state(status);
    }

    pub fn is_success(&self) -> bool {
        if let Some(last_result) = self.state.guesses.last() {
            for status in last_result {
                if match status.1 {
                    Status::Incorrect => true,
                    Status::WrongPosition => true,
                    Status::Correct => false,
                } {
                    return false;
                }
            }
            return true;
        }
        false
    }

    pub fn get_correct_word(&self) -> Option<&str> {
        if self.is_success() {
            return Some(self.state.get_correct_word());
        }
        None
    }

    pub fn invalid_guess(&self, guess: &str) -> Result<bool, String> {
        if guess.len() != self.word_length {
            Err("Guess must be of correct length.".to_string())
        } else if !self.word_set.contains(guess) {
            Err("Guess not in dictionary.".to_string())
        } else {
            Ok(true)
        }
    }

    pub fn status_display(&self) -> String {
        format!("{}", self.state)
    }

    pub fn last_status(&self) -> Option<&Vec<(char, Status)>> {
        self.state.guesses.last()
    }
}

struct GameState {
    correct_word: String,
    guesses: Vec<Vec<(char, Status)>>,
}

impl GameState {
    fn update_state(&mut self, status: Vec<(char, Status)>) {
        self.guesses.push(status);
    }

    fn get_correct_word(&self) -> &str {
        &self.correct_word
    }
}

fn print_status(guess: &[(char, Status)]) -> Result<String, std::fmt::Error> {
    let mut output = String::new();
    for (c, status) in guess {
        write!(
            &mut output,
            "{}",
            match status {
                Status::Incorrect => format!("{}", c).red(),
                Status::WrongPosition => format!("{}", c).yellow(),
                Status::Correct => format!("{}", c).green(),
            }
        )?;
    }
    Ok(output)
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.guesses.is_empty() {
            return <String as fmt::Display>::fmt(&"".to_owned(), f);
        }

        for guess in &self.guesses {
            writeln!(f, "{}", print_status(guess)?)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use colored::Colorize;

    use crate::Status;

    use super::GameState;

    #[test]
    fn test_state_fmt() {
        let guesses = vec![vec![('a', Status::Correct)]];
        let mut state = GameState {
            correct_word: "a".to_string(),
            guesses,
        };
        assert_eq!(
            format!("{}", state),
            format!("{}\n", format!("a").green().to_string())
        );
        state.update_state(vec![
            ('a', Status::Incorrect),
            ('b', Status::WrongPosition),
            ('c', Status::Correct),
        ]);
        assert_eq!(
            format!("{}", state),
            format!(
                "{}\n{second}\n",
                format!("a").green(),
                second = format_args!(
                    "{}{}{}",
                    format!("a").red(),
                    format!("b").yellow(),
                    format!("c").green()
                )
            )
        );
    }
}
