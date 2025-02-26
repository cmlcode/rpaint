use super::color::{Color, create_empty_color};

#[derive(Debug, Clone, Copy)]
pub struct Pixel{
    pub curr_color: Color,
}

pub fn create_pixel() -> Pixel {
    Pixel {curr_color:create_empty_color()}
}

pub fn create_empty_pixel() -> Pixel {
    Pixel {curr_color:create_empty_color()}

}

#[cfg(test)]
mod pixel_tests{
    use super::*;
    use crate::canvas::color::create_rgb_color;

    #[test]
    fn test_pixel_struct() {
        let curr_color:Color = create_rgb_color(0,0,0);
        let pixel = Pixel { curr_color };
        
        assert_eq!(pixel.curr_color, curr_color);
    }

    #[test]
    fn test_create_pixel_correct() {
        let empty_color:Color = create_empty_color();

        let pixel = create_pixel();
        
        assert_eq!(pixel.curr_color, empty_color);
    }

    #[test]
    fn test_create_pixel_empty() {
        let empty_color:Color = create_empty_color();

        let pixel = create_pixel();
        
        assert_eq!(pixel.curr_color, empty_color);
    }
}
