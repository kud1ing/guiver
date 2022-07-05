use crate::font::Font;

use crate::widget::{WidgetCommand, WidgetId};
use crate::{SizeConstraints, UserEvent, Widget, WidgetEvent};
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
    pub fn set_text(&mut self, text: impl Into<String>) {
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

    fn handle_user_event(&mut self, event: &UserEvent, widget_events: &mut Vec<WidgetEvent>) {
        let rectangle = Rect::from_origin_size(self.origin, self.size);

        match event {
            UserEvent::MouseDown(mouse_event) => {
                if !rectangle.contains(mouse_event.pos) {
                    return;
                }

                widget_events.push(WidgetEvent::Clicked(self.widget_id));
            }
            UserEvent::MouseMove(mouse_event) => {
                if rectangle.contains(mouse_event.pos) {
                    // TODO
                } else {
                    // TODO
                }
            }
            UserEvent::MouseUp(mouse_event) => {
                if rectangle.contains(mouse_event.pos) {
                    // TODO
                } else {
                    // TODO
                }
            }
        }
    }

    fn handle_widget_command(&mut self, widget_command: &WidgetCommand) {
        match widget_command {
            WidgetCommand::Remove(_widget_id) => {
                // A widget can not remove itself.
            }
            WidgetCommand::SetHasFocus(_widget_id, _has_focus) => {
                // A label can not have focus.
            }
            WidgetCommand::SetIsDisabled(_, _) => {
                // TODO
                println!("`Label::handle_widget_command(SetIsDisabled)`: TODO");
            }
            WidgetCommand::SetIsHidden(widget_id, is_hidden) => {
                if *widget_id == self.widget_id {
                    self.set_is_hidden(*is_hidden);
                }
            }
            WidgetCommand::SetValue(widget_id, value) => {
                if *widget_id == self.widget_id {
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

    fn set_has_focus(&mut self, _has_focus: bool) {
        // Nothing to do.
    }

    fn set_is_hidden(&mut self, is_hidden: bool) {
        self.is_hidden = is_hidden;
    }

    fn set_origin(&mut self, origin: Point) {
        self.origin = origin.into();
    }

    fn widget_id(&self) -> &WidgetId {
        &self.widget_id
    }
}
