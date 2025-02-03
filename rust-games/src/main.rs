//mod hangman;
use eframe::egui;
use std::collections::HashSet;
mod style;
//use hangman::run_hangman;
// fn main() {
//     run_hangman()
// }

//game logic
struct HangmanGame{
    secret_word: String,
    guessed_letters: HashSet<char>,
    remaining_attempts: u8,
}
impl HangmanGame{
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


//GUI
enum Screen {
    Home,
    Game,
}

struct MyApp {
    current_screen: Screen,
    game: Option<HangmanGame>, //game instance
    guess_input: String,
}

impl MyApp {
    fn new() -> Self {
        MyApp {
            current_screen: Screen::Home,
            game: None,
            guess_input: String::new(),
        }
    }

}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let apply_style = style::styles();  //applying styles
        ctx.set_style(apply_style);
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_screen {
                Screen::Home => {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.heading("Welcome to Hangman!");
                        if ui.button("Play").clicked() {
                            self.game = Some(HangmanGame::new("rust"));
                            self.current_screen = Screen::Game;
                        }
                    });
                }
                Screen::Game => {
                    // Store action results outside of UI closure
                    let mut guessed_char: Option<char> = None;
                    let mut reset_game = false;

                    if let Some(game) = &self.game {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.label(game.display_hangman());
                            ui.label(game.display_progress());
                            ui.label(format!("Remaining attempts: {}", game.remaining_attempts));

                            if game.is_won() {
                                ui.label("🎉 Congratulations! You won!");
                            } else if game.is_lost() {
                                ui.label("💀 Game Over!");
                            } else {
                                ui.horizontal(|ui| {
                                    ui.label("Guess a letter:");
                                    ui.text_edit_singleline(&mut self.guess_input);
                                    if ui.button("Guess").clicked() {
                                        guessed_char = self.guess_input.trim().chars().next();
                                    }
                                });
                            }

                            if ui.button("🔙").clicked() {
                                reset_game = true;
                            }
                        });
                    }

                    // Apply game logic after UI 
                    if let Some(game) = &mut self.game {
                        if let Some(c) = guessed_char {
                            if c.is_alphabetic() {
                                game.guess(c);
                            }
                            self.guess_input.clear();
                        }
                    }

                    if reset_game {
                        self.current_screen = Screen::Home;
                        self.game = None; 
                    }
                }
            }
        });
    }
}


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Hangman", options, Box::new(|_cc| Ok(Box::new(MyApp::new()))))
}
