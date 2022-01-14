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

    let mut game = Game::new(word_length, dictionary);
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

    let mut game = Game::new(word_length, dictionary);
    println!("Ready!");

    let solution = solve_game(&mut game);
    print!("{}", game.status_display());
    println!("Correct you guessed the word: {}", solution);
}

fn main() {
    robot_play_game();
    cmd_play_game();
}
