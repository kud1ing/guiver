use crate::font::Font;
use crate::widget::WidgetCommand;
use crate::{BoxConstraints, UserEvent, Widget, WidgetEvent};
use druid_shell::kurbo::Rect;
use druid_shell::piet::{Piet, PietTextLayout, RenderContext, TextLayout};
use druid_shell::Region;
use std::any::Any;

/// The commands a label can handle.
pub enum LabelCommand {
    SetFont(Font),
    SetPostion((f64, f64)),
    SetText(String),
}

/// The requests a label can answer.
pub enum LabelRequest {
    GetRectangle,
    PreferredSize(BoxConstraints),
}

/// A label.
pub struct Label {
    font: Font,
    is_hidden: bool,
    position: (f64, f64),
    rectangle: Rect,
    text: String,
    text_layout: PietTextLayout,
}

impl Label {
    pub fn new(text: impl Into<String>, commands: Vec<Box<dyn Any>>) -> Self {
        let font = Font::default();
        let text = text.into();

        let mut label = Label {
            font,
            is_hidden: false,
            position: (0.0, 0.0),
            rectangle: Rect::default(),
            text: text.clone(),
            text_layout: Font::default().text_layout(text),
        };

        // Handle the given commands.
        label.handle_commands(commands);

        label
    }

    /// Handles the given label command.
    fn handle_label_command(&mut self, label_command: &LabelCommand) {
        match label_command {
            LabelCommand::SetFont(font) => {
                self.font = font.clone();
            }
            LabelCommand::SetPostion(position) => {
                self.position = position.clone();
            }
            LabelCommand::SetText(text) => {
                self.text = text.clone();
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

impl Widget for Label {
    fn handle_commands(&mut self, commands: Vec<Box<dyn Any>>) {
        // Iterate over the given commands.
        for command in commands {
            // The given command is a widget command.
            if let Some(command) = command.downcast_ref::<WidgetCommand>() {
                self.handle_widget_command(command);
            }
            // The given command is a label command.
            else if let Some(command) = command.downcast_ref::<LabelCommand>() {
                self.handle_label_command(command);
            } else {
                // TODO: Error handling
            }
        }

        self.text_layout = self.font.text_layout(self.text.clone());
        self.rectangle = Rect::from_origin_size(self.position, self.text_layout.size());
    }

    fn handle_request(&mut self, _request: Box<dyn Any>) -> Option<Box<dyn Any>> {
        // TODO
        None
    }

    fn handle_user_event(&mut self, event: &UserEvent) -> Option<WidgetEvent> {
        match event {
            UserEvent::MouseDown(mouse_event) => {
                if !self.rectangle.contains(mouse_event.pos) {
                    return None;
                }

                return Some(WidgetEvent::Clicked);
            }
            UserEvent::MouseMove(mouse_event) => {
                if self.rectangle.contains(mouse_event.pos) {
                    // TODO
                } else {
                    // TODO
                }
            }
            UserEvent::MouseUp(mouse_event) => {
                if self.rectangle.contains(mouse_event.pos) {
                    // TODO
                } else {
                    // TODO
                }
            }
        }

        None
    }

    fn paint(&self, piet: &mut Piet, _region: &Region) {
        // The label is hidden.
        if self.is_hidden {
            return;
        }

        // TODO: Check the region.

        // Draw the text.
        piet.draw_text(&self.text_layout, self.position);
    }
}
