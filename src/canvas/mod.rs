pub mod pixel;
pub mod color;

use pixel::{Pixel, create_empty_pixel};

#[derive(Debug, PartialEq)]
pub enum Mode {
    Draw,
}

pub struct Canvas{
    pub dimensions: (u16, u16),
    pub mode: Mode,
    pub pixels: Vec<Pixel>
}

fn create_pixels_vector(width: u16, height: u16) -> Vec<Pixel>{
    let default_value = create_empty_pixel();
    vec![default_value; (width*height) as usize]
}
pub fn create_canvas(width: u16, height:u16) -> Canvas {
    let pixels = create_pixels_vector(width, height);
    Canvas {
        dimensions: (width, height),
        mode: Mode::Draw,
        pixels
    }
}

fn get_pixel_count(canvas: &Canvas, x_val: u16, y_val: u16) -> u32{
    let pixel_count:u32 = ((y_val * canvas.dimensions.0) + x_val).into();
    pixel_count
}

#[cfg(test)]
mod canvas_tests{
    use super::*;

    #[test]
    fn test_create_canvas() {
        let canvas: Canvas = create_canvas(5,10);
        assert_eq!(canvas.dimensions, (5,10));
        assert_eq!(canvas.mode, Mode::Draw);
        assert_eq!(canvas.pixels.len(), 50) 
    }

    #[test]
    fn test_get_pixel_count_0() {
        let canvas: Canvas = create_canvas(5,5);
        let pixel_count = get_pixel_count(&canvas, 0, 0);
        assert_eq!(pixel_count, 0);
        assert!(canvas.pixels.len() >= pixel_count as usize);
    }

    #[test]
    fn test_get_pixel_count_end_row_1() {
        let canvas: Canvas = create_canvas(5,5);
        let pixel_count = get_pixel_count(&canvas, 4, 0);
        assert_eq!(pixel_count, 4);
        assert!(canvas.pixels.len() >= pixel_count as usize);
    }

    #[test]
    fn test_get_pixel_count_end_last_row() {
        let canvas: Canvas = create_canvas(5,5);
        let pixel_count = get_pixel_count(&canvas, 4, 4);
        assert_eq!(pixel_count, 24);
        assert!(canvas.pixels.len() - 1 == pixel_count as usize);
    }

    #[test]
    fn test_get_pixel_count_mid_row() {
        let canvas: Canvas = create_canvas(5,5);
        let pixel_count = get_pixel_count(&canvas, 1, 2);
        assert_eq!(pixel_count, 11);
        assert!(canvas.pixels.len() >= pixel_count as usize);
    }
}
