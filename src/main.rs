use colored::Colorize;
use wordle_clone::{
    dictionary::{load_dictionary, Dictionary},
    guess_status, invalid_guess, is_success, Status,
};

fn print_current_status(guesses: &Vec<(String, Vec<Status>)>) {
    for guess in guesses {
        print_status(&guess.0, &guess.1);
    }
}

fn print_status(guess: &str, statuses: &Vec<Status>) {
    let mut count = 0;
    for c in guess.chars() {
        let status = &statuses[count];
        let char_out = match status {
            Status::Incorrect => format!("{}", c).red(),
            Status::WrongPosition => format!("{}", c).yellow(),
            Status::Correct => format!("{}", c).green(),
        };
        print!("{}", char_out);
        count += 1;
    }
    println!();
}

fn get_word_length() -> usize {
    let mut word_length_str = String::new();
    println!("Enter Length of word you'd like to play with :");
    std::io::stdin()
        .read_line(&mut word_length_str)
        .expect("Failed to read line.");

    word_length_str
        .trim()
        .parse::<usize>()
        .expect("Expected a positive number.")
}

fn get_new_guess(word_length: usize, dictionary: &Dictionary) -> Option<String> {
    if let Some(set) = dictionary.get_set(word_length) {
        loop {
            let mut guess = String::new();
            println!("Enter guess:");
            std::io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line.");
            let trimmed_guess = guess.trim().to_lowercase();

            if invalid_guess(&trimmed_guess, word_length, set) {
                return Some(trimmed_guess);
            }
        }
    } else {
        None
    }
}

fn play_game(dictionary: &Dictionary) {
    println!("Welcome to wordle!");

    let word_length = get_word_length();

    let word = dictionary.gen_word(word_length);
    println!("Ready!");

    let mut prev_guesses = vec![];
    loop {
        let guess = get_new_guess(word_length, dictionary);
        let current_status = guess_status(&guess.clone().unwrap(), &word);

        if is_success(&current_status) {
            break;
        }

        prev_guesses.push((guess.unwrap(), current_status));
        print_current_status(&prev_guesses);
    }

    println!("Correct you guessed the word: {}", word);
}

fn main() {
    let dictionary = load_dictionary("./resources/linuxwords");
    play_game(&dictionary);
}
