use crate::font::Font;

use crate::widget::{WidgetCommand, WidgetError, WidgetId};
use crate::{SizeConstraints, SystemEvent, Widget, WidgetEvent};
use druid_shell::kurbo::{Point, Rect, Size};
use druid_shell::piet::{PaintBrush, Piet, PietTextLayout, RenderContext, TextLayout};
use druid_shell::{piet, Region};

/// A text widget.
pub struct Text {
    debug_rendering: bool,
    debug_rendering_stroke_brush: PaintBrush,
    debug_rendering_stroke_width: f64,
    font: Font,
    is_hidden: bool,
    rectangle: Rect,
    text: String,
    text_layout: PietTextLayout,
    widget_id: WidgetId,
}

impl Text {
    ///
    pub fn new(
        widget_id: WidgetId,
        debug_rendering_stroke_brush: PaintBrush,
        debug_rendering_stroke_width: f64,
        text: impl Into<String>,
    ) -> Self {
        let font = Font::default();
        let text = text.into();

        Text {
            debug_rendering: false,
            debug_rendering_stroke_brush,
            debug_rendering_stroke_width,
            font: font.clone(),
            is_hidden: false,
            rectangle: Rect::default(),
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

impl Widget for Text {
    ///
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        // Adjust the text layout size to the given constraints.
        self.rectangle = self.rectangle.with_size(
            self.text_layout
                .size()
                .clamp(self.text_layout.size(), *size_constraints.maximum()),
        );

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
            WidgetCommand::SetDebugRendering(debug_rendering) => {
                self.debug_rendering = debug_rendering;
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
        match system_event {
            SystemEvent::KeyDown(_) => {}
            SystemEvent::KeyUp(_) => {}
            SystemEvent::MouseDown(mouse_event) => {
                if !self.rectangle.contains(mouse_event.pos) {
                    return;
                }

                widget_events.push(WidgetEvent::Clicked(self.widget_id));
            }
            SystemEvent::MouseMove(mouse_event) => {
                if self.rectangle.contains(mouse_event.pos) {
                    // TODO
                } else {
                    // TODO
                }
            }
            SystemEvent::MouseUp(mouse_event) => {
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
                &self.debug_rendering_stroke_brush,
                self.debug_rendering_stroke_width,
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
