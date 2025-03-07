mod widgets;

use eframe::egui;
use crate::canvas::{create_canvas, Canvas};
use crate::gui::widgets::canvas::CanvasWidget;

pub struct RPaint {
    canvas: Canvas,
}

impl Default for RPaint{
    fn default() -> Self {
        Self {
            canvas: create_canvas(100,75).expect("canvas should be created"),
        }
    }
}

impl eframe::App for RPaint{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(CanvasWidget { canvas: &mut self.canvas });
        });
    }
}

pub fn run() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "rPaint",
        native_options,
        Box::new(|_cc| Ok(Box::new(RPaint::default()))),
    )
}
