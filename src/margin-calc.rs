mod api;
mod gui;

use gui::App;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Margin Calc",
        options,
        Box::new(|_cc| Box::new(App::default())),
    );
}
