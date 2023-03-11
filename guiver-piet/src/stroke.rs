use druid_shell::piet::{Color, PaintBrush, StrokeStyle};

///
#[derive(Clone, Debug)]
pub struct Stroke {
    pub stroke_brush: PaintBrush,
    pub stroke_style: StrokeStyle,
    pub stroke_width: f64,
}

impl Default for Stroke {
    fn default() -> Self {
        Stroke {
            stroke_brush: PaintBrush::Color(Color::rgb8(0, 0, 0)),
            stroke_style: StrokeStyle::default(),
            stroke_width: 1.0,
        }
    }
}
