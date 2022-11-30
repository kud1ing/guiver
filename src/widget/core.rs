use crate::widget::{WidgetCommand, WidgetError};
use crate::{SizeConstraints, Stroke, WidgetId};
use druid_shell::kurbo::Rect;

/// Common data and functionality for widgets.
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

    ///
    pub fn handle_command(&mut self, widget_command: &WidgetCommand) -> Result<(), WidgetError> {
        match widget_command {
            WidgetCommand::SetDebugRendering(debug_rendering) => {
                self.debug_rendering = *debug_rendering;
                Ok(())
            }
            WidgetCommand::SetIsHidden(is_hidden) => {
                // Hide/show this widget.
                self.is_hidden = *is_hidden;
                Ok(())
            }
            _ => Err(WidgetError::NotHandled),
        }
    }
}
