use crate::widget::WidgetCommand;
use crate::{UserEvent, Widget, WidgetEvent};
use druid_shell::kurbo::{Line, Rect};
use druid_shell::piet::{Color, Piet, RenderContext, StrokeStyle};
use druid_shell::Region;
use std::any::Any;

/// The commands a placeholder can handle.
pub enum PlaceholderCommand {
    SetColor(Color),
    SetStrokeStyle(StrokeStyle),
}

/// A placeholder widget.
pub struct Placeholder {
    color: Color,
    is_hidden: bool,
    rectangle: Rect,
    stroke_style: StrokeStyle,
}

impl Placeholder {
    pub fn new(commands: Vec<Box<dyn Any>>) -> Self {
        let mut placeholder = Placeholder {
            color: Color::rgb8(255, 255, 255),
            is_hidden: false,
            rectangle: Rect::default(),
            stroke_style: StrokeStyle::new(),
        };

        // Handle the given commands.
        placeholder.handle_commands(commands);

        placeholder
    }

    /// Handles the given placeholder command.
    fn handle_placeholder_command(&mut self, placeholder_command: &PlaceholderCommand) {
        match placeholder_command {
            PlaceholderCommand::SetColor(color) => self.color = color.clone(),
            PlaceholderCommand::SetStrokeStyle(stroke_style) => {
                self.stroke_style = stroke_style.clone()
            }
        }
    }

    /// Handles the given widget command.
    fn handle_widget_command(&mut self, widget_command: &WidgetCommand) {
        match widget_command {
            WidgetCommand::SetHasFocus(_) => {}
            WidgetCommand::SetIsHidden(is_hidden) => self.is_hidden = is_hidden.clone(),
            WidgetCommand::SetRectangle(rectangle) => self.rectangle = rectangle.clone(),
        }
    }
}

impl Widget for Placeholder {
    fn handle_commands(&mut self, commands: Vec<Box<dyn Any>>) {
        // Iterate over the given commands.
        for command in commands {
            // The given command is a widget command.
            if let Some(command) = command.downcast_ref::<WidgetCommand>() {
                self.handle_widget_command(command);
            }
            // The given command is a placeholder command.
            else if let Some(command) = command.downcast_ref::<PlaceholderCommand>() {
                self.handle_placeholder_command(command);
            } else {
                // TODO: Error handling
            }
        }
    }

    fn handle_request(&mut self, _request: Box<dyn Any>) -> Option<Box<dyn Any>> {
        // TODO
        None
    }

    fn handle_user_event(&mut self, _event: &UserEvent) -> Option<WidgetEvent> {
        None
    }

    fn paint(&self, piet: &mut Piet, _region: &Region) {
        // The placeholder is hidden.
        if self.is_hidden {
            return;
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
    }
}
