use crate::{Color, PaintBrush, StrokeStyle};

///
#[derive(Clone, Debug)]
pub struct Stroke {
    pub brush: PaintBrush,
    pub style: StrokeStyle,
    pub width: f64,
}

impl Default for Stroke {
    fn default() -> Self {
        Stroke {
            brush: PaintBrush::Color(Color::rgb8(0, 0, 0)),
            style: StrokeStyle::default(),
            width: 1.0,
        }
    }
}
