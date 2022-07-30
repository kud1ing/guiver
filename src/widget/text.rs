use crate::font::Font;

use crate::stroke::Stroke;
use crate::widget::{WidgetCommand, WidgetError, WidgetId};
use crate::{Event, SizeConstraints, Widget, WidgetEvent};
use druid_shell::kurbo::{Point, Rect, Size};
use druid_shell::piet::{Piet, PietTextLayout, RenderContext, TextLayout};
use druid_shell::{piet, Region};

/// A text widget.
pub struct Text {
    debug_rendering: bool,
    debug_rendering_stroke: Stroke,
    font: Font,
    is_hidden: bool,
    rectangle: Rect,
    size_constraints: SizeConstraints,
    text: String,
    text_layout: PietTextLayout,
    widget_id: WidgetId,
}

impl Text {
    ///
    pub fn new(
        widget_id: WidgetId,
        debug_rendering_stroke: Stroke,
        font: Font,
        text: impl Into<String>,
    ) -> Self {
        let text = text.into();

        Text {
            debug_rendering: false,
            debug_rendering_stroke,
            font: font.clone(),
            is_hidden: false,
            rectangle: Rect::default(),
            size_constraints: SizeConstraints::default(),
            text: text.clone(),
            text_layout: font.text_layout(text),
            widget_id,
        }
    }

    ///
    fn layout(&mut self) {
        // Adjust the text layout size to the given constraints.
        self.rectangle = self.rectangle.with_size(
            self.text_layout
                .size()
                .clamp(self.text_layout.size(), *self.size_constraints.maximum()),
        );
    }
}

impl Widget for Text {
    ///
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.size_constraints = size_constraints;

        self.layout();

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
            WidgetCommand::SetHasFocus(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetFill(_) => {
                // TODO
                println!("`Text::handle_command(SetFill)`: TODO");
            }
            WidgetCommand::SetFont(font) => {
                self.font = font;
                self.text_layout = self.font.text_layout(self.text.clone());

                self.layout();
            }
            WidgetCommand::SetHorizontalAlignment(_) => {
                // TODO
                println!("`Text::handle_command(SetHorizontalAlignment)`: TODO");
            }
            WidgetCommand::SetDebugRendering(debug_rendering) => {
                self.debug_rendering = debug_rendering;
            }
            WidgetCommand::SetIsDisabled(_) => {
                // TODO
                println!("`Text::handle_command(SetIsDisabled)`: TODO");
            }
            WidgetCommand::SetIsHidden(is_hidden) => {
                self.is_hidden = is_hidden;
            }
            WidgetCommand::SetStroke(_) => {
                // TODO
                println!("`Text::handle_command(SetStroke)`: TODO");
            }
            WidgetCommand::SetValue(value) => {
                // The given value is a string.
                if let Some(string) = value.downcast_ref::<String>() {
                    self.text = string.clone();
                }
                // The given value is something else.
                else {
                    self.text = format!("{:?}", value);
                }

                self.text_layout = self.font.text_layout(self.text.clone());
                self.layout();
            }
            WidgetCommand::SetVerticalAlignment(_) => {
                // TODO
                println!("`Text::handle_command(SetVerticalAlignment)`: TODO");
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: &Event, widget_events: &mut Vec<WidgetEvent>) {
        match event {
            Event::KeyDown(_) => {}
            Event::KeyUp(_) => {}
            Event::MouseDown(mouse_event) => {
                if !self.rectangle.contains(mouse_event.pos) {
                    return;
                }

                widget_events.push(WidgetEvent::Clicked(self.widget_id));
            }
            Event::MouseMove(mouse_event) => {
                if self.rectangle.contains(mouse_event.pos) {
                    // TODO
                } else {
                    // TODO
                }
            }
            Event::MouseUp(mouse_event) => {
                if self.rectangle.contains(mouse_event.pos) {
                    // TODO
                } else {
                    // TODO
                }
            }
        }
    }

    fn paint(&self, piet: &mut Piet, _region: &Region) -> Result<(), piet::Error> {
        // The text is hidden.
        if self.is_hidden {
            return Ok(());
        }

        // TODO: Check the region.

        // Draw the text clipped.
        piet.save()?;
        piet.clip(&self.rectangle);
        piet.draw_text(&self.text_layout, self.rectangle.origin());
        piet.restore()?;

        // Render debug hints.
        if self.debug_rendering {
            piet.stroke(
                self.rectangle,
                &self.debug_rendering_stroke.brush,
                self.debug_rendering_stroke.width,
            );
        }

        Ok(())
    }

    fn set_origin(&mut self, origin: Point) {
        self.rectangle = self.rectangle.with_origin(origin);
    }

    fn widget_id(&self) -> &WidgetId {
        &self.widget_id
    }
}
