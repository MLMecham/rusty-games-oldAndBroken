use eframe::egui;
mod style; //importing the style module

enum Screen {
    Home,
    Game,
} //creating variables for screens

struct MyApp{
    current_screen: Screen,
} //application state

impl MyApp{
    fn new() ->Self{
        MyApp{
            current_screen: Screen::Home,//starts on the home screen
        }
    }
}

fn main() -> Result<(), eframe::Error> { //main function to start the GUI. Returns nothing if successful and an error if not
    let options = eframe::NativeOptions::default(); //initializes default options for the application window
    eframe::run_native( //starting the application with...
        "Hangman",
        options, //default options
        Box::new(|_cc| Ok(Box::new(MyApp::new()))), //creating an instance of MyApp inside a Box and wrapping everything in Ok() 

    )
}

impl eframe::App for MyApp { //impementing eframe for MyApp
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { //update runs every frame to draw the UI | ctx is the egui::context that allows you to create UI elements
        let apply_style = style::styles(); //setting the style
        ctx.set_style(apply_style);

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_screen 
            {

                Screen::Home => {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.heading("Welcome to Hangman!");
                            if ui.button("Play").clicked() {
                                self.current_screen = Screen::Game;
                            }
                        });
                }

                Screen::Game => {          
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.heading("Game Screen");
                            if ui.button("Go Home").clicked() {
                                self.current_screen = Screen::Home;
                            }
                        });

                }
            }




        }); //creates a central panel in the window
            
            
       
    }
}
