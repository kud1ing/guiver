use crate::widget::{WidgetCommand, WidgetId};
use crate::{SizeConstraints, UserEvent, Widget, WidgetEvent};
use druid_shell::kurbo::{Line, Point, Rect, Size};
use druid_shell::piet::{Color, Piet, RenderContext, StrokeStyle};
use druid_shell::Region;
use std::collections::HashMap;

/// A placeholder widget.
pub struct Placeholder {
    color: Color,
    is_hidden: bool,
    origin: Point,
    size: Size,
    stroke_style: StrokeStyle,
    widget_id: WidgetId,
}

impl Placeholder {
    pub fn new(widget_id: WidgetId) -> Self {
        Placeholder {
            color: Color::rgb8(255, 255, 255),
            is_hidden: false,
            origin: (0.0, 0.0).into(),
            size: Size::default(),
            stroke_style: StrokeStyle::new(),
            widget_id,
        }
    }
}

impl Widget for Placeholder {
    ///
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.size = *size_constraints.maximum();
        self.size
    }

    fn handle_commands(&mut self, widget_commands: &HashMap<WidgetId, Vec<WidgetCommand>>) {
        // There are commands for this widget.
        if let Some(widget_commands) = widget_commands.get(&self.widget_id) {
            for widget_command in widget_commands {
                match widget_command {
                    WidgetCommand::Clear => {
                        // A placeholder has no children.
                    }
                    WidgetCommand::Remove => {
                        // A widget can not remove itself.
                    }
                    WidgetCommand::SetHasFocus(_has_focus) => {
                        // Nothing to do.
                    }
                    WidgetCommand::SetIsDisabled(_) => {
                        // TODO
                        println!("`Placeholder::handle_widget_command(SetIsDisabled)`: TODO");
                    }
                    WidgetCommand::SetIsHidden(is_hidden) => {
                        self.is_hidden = *is_hidden;
                    }
                    WidgetCommand::SetValue(_value) => {
                        // Nothing to do.
                    }
                }
            }
        }
    }

    fn handle_event(&mut self, _event: &UserEvent, _widget_events: &mut Vec<WidgetEvent>) {
        // Nothing to do.
    }

    fn paint(&self, piet: &mut Piet, _region: &Region) {
        // The placeholder is hidden.
        if self.is_hidden {
            return;
        }

        // TODO: check the region

        let rectangle = Rect::from_origin_size(self.origin, self.size);

        // Draw a cross.
        piet.stroke(
            Line::new((rectangle.x0, rectangle.y0), (rectangle.x1, rectangle.y1)),
            &self.color,
            1.0,
        );
        piet.stroke(
            Line::new((rectangle.x0, rectangle.y1), (rectangle.x1, rectangle.y0)),
            &self.color,
            1.0,
        );

        // Draw the rectangle.
        piet.stroke(&rectangle, &self.color, 1.0);
    }

    fn set_origin(&mut self, origin: Point) {
        self.origin = origin;
    }

    fn widget_id(&self) -> &WidgetId {
        &self.widget_id
    }
}
