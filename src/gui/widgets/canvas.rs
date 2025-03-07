use crate::gui::egui::{Sense, Ui, Widget, Response, vec2, Rect, Painter, Pos2, Key};
use crate::canvas::{Canvas, get_pixel_xy_coords, get_pixel_count, interact_with_pixel, Mode};

/// Canvas object with pixels
pub struct CanvasWidget<'a> {
    pub canvas: &'a mut Canvas,
        
}

impl CanvasWidget<'_> {
    fn get_pixel_size(&self, rect: &Rect) -> (f32, f32) {
        let pixel_size: (f32, f32) = (
            rect.width()/self.canvas.dimensions.0 as f32,
            rect.height()/self.canvas.dimensions.1 as f32
        );
        pixel_size
    }

    fn get_pixel_from_pointer(&self, pointer_pos:Pos2, rect: &Rect) -> u32{
        let pixel_size = self.get_pixel_size(&rect);
        let pixel_x = ((pointer_pos.x - rect.min.x) / pixel_size.0) as u16;
        let pixel_y = ((pointer_pos.y - rect.min.y) / pixel_size.1) as u16;
        get_pixel_count(self.canvas, pixel_x, pixel_y)
    }

    fn draw_pixels(self, painter: &Painter, rect: &Rect) {
        for (index, color) in self.canvas.to_egui_pixels().iter().enumerate() {
            let pixel_size = self.get_pixel_size(&rect);

            let coords: (u16, u16) = get_pixel_xy_coords(self.canvas, &(index as u32));
            let pixel_x = rect.min.x + (coords.0 as f32 * pixel_size.0);
            let pixel_y = rect.min.y + (coords.1 as f32 * pixel_size.1);

            let pixel_rect = Rect::from_min_size((pixel_x, pixel_y).into(), vec2(pixel_size.0, pixel_size.1));
            painter.rect_filled(pixel_rect, 0.0, *color);
        }
    }
}

impl Widget for CanvasWidget<'_>{
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), Sense::click_and_drag());

        if ui.is_rect_visible(rect) {
            let input = ui.input(|i| i.clone());
            if input.key_pressed(Key::E) {
                self.canvas.mode = Mode::Erase;
            } else if input.key_pressed(Key::D) {
                self.canvas.mode = Mode::Draw;
            } else if input.pointer.primary_clicked() {
                if let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos()) {
                    let pixel_num:u32 = self.get_pixel_from_pointer(pointer_pos, &rect);
                    interact_with_pixel(self.canvas, &pixel_num).expect("Mode sould be real");
                }
            } else if input.pointer.primary_down() {
                if let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos()) {
                    let pixel_num:u32 = self.get_pixel_from_pointer(pointer_pos, &rect);
                    interact_with_pixel(self.canvas, &pixel_num).expect("Mode sould be real");
                }
            }


            let painter = ui.painter();
            painter.rect_filled(rect, 0.0, crate::gui::egui::Color32::WHITE);
            self.draw_pixels(&painter, &rect)
        }

        response
    }
}
