#[derive(Debug)]
pub enum CanvasError {
    CanvasTooLarge { max_pixels: u32, requested: u32 },
    FileSaveError { filepath: String }
}

impl std::fmt::Display for CanvasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CanvasError::CanvasTooLarge { max_pixels, requested } => {
                write!(f, "Requested {} pixels, but the maximum allowed is {}.", requested, max_pixels)
            },
            CanvasError::FileSaveError { filepath } => {
                write!(f, "Failed saving file with path {}.", filepath )
            }
        }
    }
}

impl std::error::Error for CanvasError {}
