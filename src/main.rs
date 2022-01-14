use std::collections::HashMap;

use pbr::ProgressBar;
use wordle_clone::{dictionary::load_dictionary, game::Game, solver::solve_game};

fn get_new_guess(game: &Game) -> String {
    loop {
        let mut guess = String::new();
        println!("Enter guess:");
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        let trimmed_guess = guess.trim().to_lowercase();

        match game.invalid_guess(&trimmed_guess) {
            Ok(_) => return trimmed_guess,
            Err(e) => println!("{}", e),
        }
    }
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

fn cmd_play_game() {
    let dictionary = load_dictionary("./resources/linuxwords");
    println!("Welcome to wordle!");

    let word_length = get_word_length();

    let mut game = Game::new(word_length, &dictionary);
    println!("Ready!");

    loop {
        let guess = get_new_guess(&game);
        game.play_game(guess);
        print!("{}", game.status_display());

        if game.is_success() {
            break;
        }
    }
    println!(
        "Correct you guessed the word: {}",
        game.get_correct_word().unwrap()
    );
}

fn robot_play_game() {
    let dictionary = load_dictionary("./resources/linuxwords");
    println!("Welcome to wordle!");

    let word_length = get_word_length();

    let mut game = Game::new(word_length, &dictionary);
    println!("Ready!");

    let solution = solve_game(&mut game);
    print!("{}", game.status_display());
    println!("Correct you guessed the word: {}", solution);
}

fn calc_avg_guesses() -> HashMap<usize, (usize, f64)> {
    let dictionary = load_dictionary("./resources/linuxwords");
    let mut counts: HashMap<usize, (usize, f64)> = HashMap::new();
    let keys = dictionary.keys();
    let amount = keys.len();
    for length in keys {
        println!(
            "Trying word length {} out of {amount}",
            length,
            amount = amount
        );
        let set_count = dictionary.get_set(*length).unwrap().len().min(100);
        let mut sum: i32 = 0;

        let mut pb = ProgressBar::new(set_count.try_into().unwrap());
        for _ in 1..set_count {
            let mut game = Game::new(*length, &dictionary);
            let _ = solve_game(&mut game);
            sum += game.number_guesses() as i32;
            pb.inc();
        }

        pb.finish_println(&format!("done with {}", length));

        counts.insert(*length, (set_count, sum as f64 / set_count as f64));
    }
    counts
}

fn main() {
    println!("{:?}", calc_avg_guesses());
    robot_play_game();
    cmd_play_game();
}
