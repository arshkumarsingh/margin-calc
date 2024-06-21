mod api;
mod gui;

use gui::App;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        Box::new(App::default()),
        options,
    );
}
