use crate::{SizeConstraints, Stroke, WidgetId};
use druid_shell::kurbo::Rect;

///
#[derive(Default)]
pub struct WidgetCore {
    pub debug_rendering: bool,
    pub debug_rendering_stroke: Stroke,
    pub is_hidden: bool,
    pub rectangle: Rect,
    pub size_constraints: SizeConstraints,
    pub widget_id: WidgetId,
}

impl WidgetCore {
    ///
    pub fn new(widget_id: WidgetId, debug_rendering_stroke: Stroke) -> Self {
        WidgetCore {
            debug_rendering: false,
            debug_rendering_stroke,
            is_hidden: false,
            rectangle: Rect::default(),
            size_constraints: SizeConstraints::unbounded(),
            widget_id,
        }
    }
}
