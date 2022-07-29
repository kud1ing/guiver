use crate::{PaintBrush, StrokeDash, StrokeStyle};

///
#[derive(Clone, Debug)]
pub struct Stroke {
    pub brush: PaintBrush,
    pub dash: Option<StrokeDash>,
    pub style: StrokeStyle,
    pub width: f64,
}
