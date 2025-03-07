pub mod pixel;
pub mod color;
pub mod error;

use std::result::Result;
use std::path::Path;
use image::{Rgba, ImageBuffer};
use pixel::{Pixel, create_empty_pixel};
use crate::canvas::color::{create_rgb_color, create_empty_color};
use error::CanvasError;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Draw,
    Erase,
}

#[derive(Debug)]
pub struct Canvas{
    pub dimensions: (u16, u16),
    pub mode: Mode,
    pub pixels: Vec<Pixel>
}

impl Canvas {
    pub fn to_egui_pixels(&self) -> Vec<eframe::egui::Color32> {
        self.pixels
            .iter()
            .map(|p| p.curr_color.to_egui_color32())
            .collect()
    }
}

fn create_pixels_vector(width: u16, height: u16) -> Vec<Pixel>{
    let default_value = create_empty_pixel();
    let pixel_count = (width as usize) * (height as usize);
    vec![default_value; pixel_count]
}

pub fn create_canvas(width: u16, height:u16) -> Result<Canvas, CanvasError> {
    let pixel_count = (width as u32) * (height as u32);
    let max_pixels = 8294400;
    if pixel_count > max_pixels {
        return Err(CanvasError::CanvasTooLarge {
            max_pixels,
            requested: pixel_count
        })
    }

    let pixels = create_pixels_vector(width, height);

    Ok( Canvas {
            dimensions: (width, height),
            mode: Mode::Draw,
            pixels
    })
}

pub fn get_pixel_xy_coords(canvas: &Canvas, pixel_count: &u32) -> (u16, u16) {
    let x_coord: u16 = (pixel_count % canvas.dimensions.0 as u32) as u16;
    let y_coord: u16 = (pixel_count / canvas.dimensions.0 as u32) as u16;
    (x_coord, y_coord)
}

pub fn get_pixel_count(canvas: &Canvas, x_val: u16, y_val: u16) -> u32{
    let y_count:u32 = y_val as u32 * canvas.dimensions.0 as u32;
    let pixel_count:u32 = y_count + x_val as u32;
    pixel_count
}

pub fn interact_with_pixel(canvas: &mut Canvas, pixel_num: &u32) -> Result<(), CanvasError> {
    match canvas.mode {
        Mode::Draw => canvas.pixels[*pixel_num as usize].curr_color = create_rgb_color(255,0,0),
        Mode::Erase => canvas.pixels[*pixel_num as usize].curr_color = create_empty_color(),
    }
    Ok(())
}

pub fn save_canvas_as_png(canvas: &Canvas, file_path: &str) -> Result<(), CanvasError> {
    let (width, height) = (canvas.dimensions.0 as u32, canvas.dimensions.1 as u32);
    let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
    for (index, pixel) in canvas.pixels.iter().enumerate() {
        let x = (index as u32) % width;
        let y = (index as u32) / width;

        let color = pixel.curr_color.to_rgba();
        img.put_pixel(x,y, Rgba([color[0], color[1], color[2], color[3]]))
    }

    img.save(Path::new(file_path))
        .map_err(|_| CanvasError::FileSaveError{ filepath: file_path.to_string() })?;

    Ok(())
}

#[cfg(test)]
mod canvas_tests{
    use super::*;

    #[test]
    fn test_create_canvas() {
        let canvas: Canvas = create_canvas(5, 10).expect("Canvas should be created");
        assert_eq!(canvas.dimensions, (5,10));
        assert_eq!(canvas.mode, Mode::Draw);
        assert_eq!(canvas.pixels.len(), 50) 
    }

    #[test]
    fn test_create_canvas_too_large() {
        let result = create_canvas(u16::MAX, u16::MAX);

        if let Err(CanvasError::CanvasTooLarge { max_pixels, requested }) = result {
            assert_eq!(max_pixels, 8294400);
            assert_eq!(requested, (u16::MAX as u32) * (u16::MAX as u32));
        } else {
            panic! ("Expected CanvasError::TooLarge, but got {:?}", result);
        }
    }

    #[test]
    fn test_create_canvas_max() {
        let canvas: Canvas = create_canvas(3840, 2160).expect("Canvas should be created");
        assert_eq!(canvas.dimensions, (3840,2160));
        assert_eq!(canvas.mode, Mode::Draw);
        assert_eq!(canvas.pixels.len(), (3840 as usize * 2160 as usize))
    }

    #[test]
    fn test_get_pixel_count_0() {
        let canvas: Canvas = create_canvas(5,5).expect("Canvas should be created");
        let pixel_count = get_pixel_count(&canvas, 0, 0);
        assert_eq!(pixel_count, 0);
        assert!(canvas.pixels.len() >= pixel_count as usize);
    }

    #[test]
    fn test_get_pixel_count_end_row_1() {
        let canvas: Canvas = create_canvas(5,5).expect("Canvas should be created");
        let pixel_count = get_pixel_count(&canvas, 4, 0);
        assert_eq!(pixel_count, 4);
        assert!(canvas.pixels.len() >= pixel_count as usize);
    }

    #[test]
    fn test_get_pixel_count_end_last_row() {
        let canvas: Canvas = create_canvas(5,5).expect("Canvas should be created");
        let pixel_count = get_pixel_count(&canvas, 4, 4);
        assert_eq!(pixel_count, 24);
        assert!(canvas.pixels.len() - 1 == pixel_count as usize);
    }

    #[test]
    fn test_get_pixel_count_mid_row() {
        let canvas: Canvas = create_canvas(5,5).expect("Canvas should be created");
        let pixel_count = get_pixel_count(&canvas, 1, 2);
        assert_eq!(pixel_count, 11);
        assert!(canvas.pixels.len() >= pixel_count as usize);
    }
}
