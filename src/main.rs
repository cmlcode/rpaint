mod canvas;
mod gui;

fn main() {
    if let Err(e) = gui::run() {
        eprintln!("Error running application: {}", e);
    }
}
