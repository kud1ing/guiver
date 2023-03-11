use crate::font::Font;
use crate::stroke::Stroke;
use druid_shell::piet::{Color, PaintBrush, StrokeStyle};
use guiver::VerticalAlignment;

///
#[derive(Clone)]
pub struct Style {
    pub accent_color: Color,
    pub debug_rendering_stroke: Stroke,
    pub font: Font,
    pub frame_color: Color,
    pub padding: f64,
    pub spacing: f64,
    pub vertical_alignment: VerticalAlignment,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            accent_color: Color::rgb8(255, 200, 0),
            debug_rendering_stroke: Stroke {
                stroke_brush: PaintBrush::Color(Color::rgb8(255, 0, 0)),
                stroke_style: StrokeStyle::default(),
                stroke_width: 1.0,
            },
            font: Font::default(),
            frame_color: Color::rgb8(120, 120, 120),
            padding: 15.0,
            spacing: 8.0,
            vertical_alignment: VerticalAlignment::Middle,
        }
    }
}
