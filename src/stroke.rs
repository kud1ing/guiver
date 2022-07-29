use crate::{PaintBrush, StrokeStyle};

///
#[derive(Clone, Debug)]
pub struct Stroke {
    pub brush: PaintBrush,
    pub style: StrokeStyle,
    pub width: f64,
}
