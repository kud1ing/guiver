use crate::VerticalAlignment;
use druid_shell::piet::{Color, PaintBrush};

///
#[derive(Clone)]
pub struct Style {
    pub accent_color: Color,
    pub debug_rendering_stroke_brush: PaintBrush,
    pub debug_rendering_stroke_width: f64,
    pub frame_color: Color,
    pub padding: f64,
    pub spacing: f64,
    pub vertical_alignment: VerticalAlignment,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            accent_color: Color::rgb8(255, 200, 0),
            debug_rendering_stroke_brush: PaintBrush::Color(Color::rgb8(255, 0, 0)),
            debug_rendering_stroke_width: 1.0,
            frame_color: Color::rgb8(150, 150, 150),
            padding: 15.0,
            spacing: 8.0,
            vertical_alignment: VerticalAlignment::Middle,
        }
    }
}
