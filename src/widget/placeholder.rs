use crate::widget::{WidgetCommand, WidgetError, WidgetId};
use crate::{SizeConstraints, SystemEvent, Widget, WidgetEvent};
use druid_shell::kurbo::{Line, Point, Rect, Size};
use druid_shell::piet::{Color, Piet, RenderContext};
use druid_shell::{piet, Region};

/// A placeholder widget.
pub struct Placeholder {
    color: Color,
    is_hidden: bool,
    rectangle: Rect,
    widget_id: WidgetId,
}

impl Placeholder {
    pub fn new(widget_id: WidgetId) -> Self {
        Placeholder {
            color: Color::rgb8(255, 255, 255),
            is_hidden: false,
            rectangle: Rect::default(),
            widget_id,
        }
    }
}

impl Widget for Placeholder {
    ///
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.rectangle = self.rectangle.with_size(*size_constraints.maximum());
        self.rectangle.size()
    }

    fn handle_command(&mut self, widget_command: WidgetCommand) -> Result<(), WidgetError> {
        match widget_command {
            WidgetCommand::AppendChild(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::RemoveAllChildren => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::RemoveChild(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetDebugRendering(_debug_rendering) => {
                // Debug rendering is unnecessary for placeholder widgets.
            }
            WidgetCommand::SetHasFocus(_has_focus) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetIsDisabled(_) => {
                // TODO
                println!("`Placeholder::handle_widget_command(SetIsDisabled)`: TODO");
            }
            WidgetCommand::SetIsHidden(is_hidden) => {
                self.is_hidden = is_hidden;
            }
            WidgetCommand::SetValue(ref _value) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, _event: &SystemEvent, _widget_events: &mut Vec<WidgetEvent>) {
        // Nothing to do.
    }

    fn paint(&self, piet: &mut Piet, _region: &Region) -> Result<(), piet::Error> {
        // The placeholder is hidden.
        if self.is_hidden {
            return Ok(());
        }

        // TODO: check the region

        // Draw a cross.
        piet.stroke(
            Line::new(
                (self.rectangle.x0, self.rectangle.y0),
                (self.rectangle.x1, self.rectangle.y1),
            ),
            &self.color,
            1.0,
        );
        piet.stroke(
            Line::new(
                (self.rectangle.x0, self.rectangle.y1),
                (self.rectangle.x1, self.rectangle.y0),
            ),
            &self.color,
            1.0,
        );

        // Draw the rectangle.
        piet.stroke(&self.rectangle, &self.color, 1.0);

        Ok(())
    }

    fn set_origin(&mut self, origin: Point) {
        self.rectangle = self.rectangle.with_origin(origin)
    }

    fn widget_id(&self) -> &WidgetId {
        &self.widget_id
    }
}
