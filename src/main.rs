mod gui;
mod canvas;

use gui::CanvasGui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Canvas",
        native_options,
        Box::new(|_cc| Ok(Box::new(CanvasGui::default()))),
    );
}
