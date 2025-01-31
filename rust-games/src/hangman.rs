use std::collections::HashSet;
use std::io;

struct HangmanGame {
    secret_word: String,
    guessed_letters: HashSet<char>,
    remaining_attempts: u8,
}

impl HangmanGame {
    const HANGMAN_STAGES: [&'static str; 7] = [
        "
          +---+
          |   |
              |
              |
              |
              |
        =========",
        "
          +---+
          |   |
          O   |
              |
              |
              |
        =========",
        "
          +---+
          |   |
          O   |
          |   |
              |
              |
        =========",
        "
          +---+
          |   |
          O   |
         /|   |
              |
              |
        =========",
        "
          +---+
          |   |
          O   |
         /|\\  |
              |
              |
        =========",
        "
          +---+
          |   |
          O   |
         /|\\  |
         /    |
              |
        =========",
        "
          +---+
          |   |
          O   |
         /|\\  |
         / \\  |
              |
        =========",
    ];

    fn new(secret_word: &str) -> Self {
        HangmanGame {
            secret_word: secret_word.to_lowercase(),
            guessed_letters: HashSet::new(),
            remaining_attempts: 6,
        }
    }

    fn display_progress(&self) -> String {
        self.secret_word
            .chars()
            .map(|c| {
                if self.guessed_letters.contains(&c) {
                    c
                } else {
                    '_'
                }
            })
            .collect()
    }

    fn guess(&mut self, c: char) -> bool {
        let c = c.to_ascii_lowercase();
        self.guessed_letters.insert(c);
        
        if !self.secret_word.contains(c) {
            self.remaining_attempts -= 1;
            false
        } else {
            true
        }
    }

    fn is_won(&self) -> bool {
        self.secret_word
            .chars()
            .all(|c| self.guessed_letters.contains(&c))
    }

    fn is_lost(&self) -> bool {
        self.remaining_attempts == 0
    }

    fn display_hangman(&self) -> &'static str {
        HangmanGame::HANGMAN_STAGES[(6 - self.remaining_attempts) as usize]
    }
}

pub fn run_hangman() {
    let secret_word = "rust";
    let mut game = HangmanGame::new(&secret_word);    

    
    println!("Welcome to Hangman!");

    while !game.is_won() && !game.is_lost() {
        println!("{}", game.display_hangman());
        println!("Word: {}", game.display_progress());
        println!("Guesses remaining: {}", game.remaining_attempts);
        println!("Enter a letter:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if let Some(c) = input.trim().chars().next() {
            if !c.is_alphabetic() {
                println!("Please enter a valid letter!");
                continue;
            }

            let already_guessed = !game.guessed_letters.insert(c);
            if already_guessed {
                println!("You already guessed that letter!");
                continue;
            }

            if !game.guess(c) {
                println!("Incorrect guess!");
            }
        }
    }

    println!("{}", game.display_hangman());
    
    if game.is_won() {
        println!("Congratulations! You won! The word was: {}", game.secret_word);
    } else {
        println!("Game over! The word was: {}", game.secret_word);
    }
}