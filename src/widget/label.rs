use crate::font::Font;

use crate::widget::{WidgetCommand, WidgetError, WidgetId};
use crate::{SizeConstraints, SystemEvent, Widget, WidgetEvent};
use druid_shell::kurbo::{Point, Rect, Size};
use druid_shell::piet::{Piet, PietTextLayout, RenderContext, TextLayout};
use druid_shell::Region;

/// A label.
pub struct Label {
    font: Font,
    is_hidden: bool,
    origin: Point,
    size: Size,
    text: String,
    text_layout: PietTextLayout,
    widget_id: WidgetId,
}

impl Label {
    ///
    pub fn new(widget_id: WidgetId, text: impl Into<String>) -> Self {
        let font = Font::default();
        let text = text.into();

        Label {
            font: font.clone(),
            is_hidden: false,
            origin: (0.0, 0.0).into(),
            size: Size::default(),
            text: text.clone(),
            text_layout: font.text_layout(text),
            widget_id,
        }
    }

    ///
    fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.text_layout = self.font.text_layout(self.text.clone());
    }
}

impl Widget for Label {
    ///
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        // Adjust the text layout size to the given constraints.
        self.size = self
            .text_layout
            .size()
            .clamp(*size_constraints.minimum(), *size_constraints.maximum());

        self.size
    }

    fn handle_command(&mut self, widget_command: WidgetCommand) -> Result<(), WidgetError> {
        match widget_command {
            WidgetCommand::AppendChild(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::Clear => {
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
            WidgetCommand::SetHasFocus(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetIsDisabled(_) => {
                // TODO
                println!("`Label::handle_widget_command(SetIsDisabled)`: TODO");
            }
            WidgetCommand::SetIsHidden(is_hidden) => {
                self.is_hidden = is_hidden;
            }
            WidgetCommand::SetValue(value) => {
                // The given value is a string.
                if let Some(string) = value.downcast_ref::<String>() {
                    self.set_text(string);
                }
                // The given value is something else.
                else {
                    self.set_text(format!("{:?}", value));
                }
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, system_event: &SystemEvent, widget_events: &mut Vec<WidgetEvent>) {
        let rectangle = Rect::from_origin_size(self.origin, self.size);

        match system_event {
            SystemEvent::MouseDown(mouse_event) => {
                if !rectangle.contains(mouse_event.pos) {
                    return;
                }

                widget_events.push(WidgetEvent::Clicked(self.widget_id));
            }
            SystemEvent::MouseMove(mouse_event) => {
                if rectangle.contains(mouse_event.pos) {
                    // TODO
                } else {
                    // TODO
                }
            }
            SystemEvent::MouseUp(mouse_event) => {
                if rectangle.contains(mouse_event.pos) {
                    // TODO
                } else {
                    // TODO
                }
            }
        }
    }

    fn paint(&self, piet: &mut Piet, _region: &Region) {
        // The label is hidden.
        if self.is_hidden {
            return;
        }

        // TODO: Check the region.

        // TODO: clip to the size.

        // Draw the text.
        piet.draw_text(&self.text_layout, self.origin);
    }

    fn set_origin(&mut self, origin: Point) {
        self.origin = origin.into();
    }

    fn widget_id(&self) -> &WidgetId {
        &self.widget_id
    }
}
