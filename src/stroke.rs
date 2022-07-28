use druid_shell::piet::{PaintBrush, StrokeStyle};
use piet::StrokeDash;

///
#[derive(Clone)]
pub struct Stroke {
    pub brush: PaintBrush,
    pub dash: Option<StrokeDash>,
    pub style: StrokeStyle,
    pub width: f64,
}
