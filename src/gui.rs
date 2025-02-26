use eframe::egui;
use crate::canvas::{Canvas, create_canvas};

pub struct CanvasGui {
    canvas: Canvas,
}

impl Default for CanvasGui {
    fn default() -> Self {
        Self {
            canvas: create_canvas(10,10),
        }
    }
}

impl eframe::App for CanvasGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Canvas");
        });
    }
}
