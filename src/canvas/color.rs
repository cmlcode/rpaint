use eframe::egui::Color32;
use image::Rgb;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub color: Option<Rgb<u8>>
}

impl Color {
    pub fn to_egui_color32(&self) -> Color32 {
        match self.color {
            Some(Rgb([r, g, b])) => Color32::from_rgb(r, g, b),
            None => Color32::TRANSPARENT,
        }
    }
}

pub fn create_empty_color() -> Color{
    Color{ color: None }
}

pub fn create_rgb_color(red:u8,green:u8,blue:u8) -> Color {
    let color:Rgb<u8> = image::Rgb([red, green, blue]);
    Color{ color: Some(color) }
}

#[cfg(test)]
mod color_tests{
    use super::*;

    #[test]
    fn test_struct_creation_min_vals() {
        let color:Rgb<u8> = image::Rgb([0, 0, 0]);
        let color_obj = Color { color: Some(color) };
        let true_color: Rgb<u8> = color_obj.color.expect("Color should be Rgb<u8>");

        assert_eq!(true_color[0], 0);
        assert_eq!(true_color[1], 0);
        assert_eq!(true_color[2], 0);
    }

    #[test]
    fn test_struct_creation_max_vals() {
        let color:Rgb<u8> = image::Rgb([255, 255, 255]);
        let color_obj = Color { color: Some(color) };
        let true_color: Rgb<u8> = color_obj.color.expect("Color should be Rgb<u8>");

        assert_eq!(true_color[0], 255);
        assert_eq!(true_color[1], 255);
        assert_eq!(true_color[2], 255);
    }

    #[test]
    fn test_struct_creation_red() {
        let color:Rgb<u8> = image::Rgb([255, 0, 0]);
        let color_obj = Color { color: Some(color) };
        let true_color: Rgb<u8> = color_obj.color.expect("Color should be Rgb<u8>");

        assert_eq!(true_color[0], 255);
        assert_eq!(true_color[1], 0);
        assert_eq!(true_color[2], 0);
    }

    #[test]
    fn test_struct_creation_green() {
        let color:Rgb<u8> = image::Rgb([0, 255, 0]);
        let color_obj = Color { color: Some(color) };
        let true_color: Rgb<u8> = color_obj.color.expect("Color should be Rgb<u8>");

        assert_eq!(true_color[0], 0);
        assert_eq!(true_color[1], 255);
        assert_eq!(true_color[2], 0);
    }

    #[test]
    fn test_struct_creation_blue() {
        let color:Rgb<u8> = image::Rgb([0, 0, 255]);
        let color_obj = Color { color: Some(color) };
        let true_color: Rgb<u8> = color_obj.color.expect("Color should be Rgb<u8>");

        assert_eq!(true_color[0], 0);
        assert_eq!(true_color[1], 0);
        assert_eq!(true_color[2], 255);
    }

    #[test]
    fn test_struct_creation_empty() {
        let color_obj = Color { color: None };
        assert!(color_obj.color.is_none());
    }

    #[test]
    fn test_create_empty_color() {
        let color_obj = create_empty_color();
        assert!(color_obj.color.is_none());
    }

    #[test]
    fn test_create_rgb_color() {
        let red:u8 = 100;
        let green:u8 = 0;
        let blue:u8 = 255;

        let color_obj = create_rgb_color(red, green, blue);
        let true_color: Rgb<u8> = color_obj.color.expect("Color should be Rgb<u8>");

        assert_eq!(true_color[0], 100);
        assert_eq!(true_color[1], 0);
        assert_eq!(true_color[2], 255);
    }

}
