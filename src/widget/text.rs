use crate::font::Font;

use crate::stroke::Stroke;
use crate::widget::core::WidgetCore;
use crate::widget::{WidgetCommand, WidgetError, WidgetId};
use crate::{Event, HorizontalAlignment, SizeConstraints, VerticalAlignment, Widget, WidgetEvent};
use druid_shell::kurbo::{Point, Rect, Size};
use druid_shell::piet::{Piet, PietTextLayout, RenderContext, TextLayout};
use druid_shell::{piet, Region};

/// A text widget.
pub struct Text {
    core: WidgetCore,
    font: Font,
    horizontal_alignment: HorizontalAlignment,
    is_hidden: bool,
    rectangle: Rect,
    size_constraints: SizeConstraints,
    text: String,
    text_layout: PietTextLayout,
    text_origin: Point,
    vertical_alignment: VerticalAlignment,
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
            core: WidgetCore::new(widget_id, debug_rendering_stroke),
            font: font.clone(),
            horizontal_alignment: HorizontalAlignment::Center,
            is_hidden: false,
            rectangle: Rect::default(),
            size_constraints: SizeConstraints::default(),
            text: text.clone(),
            text_layout: font.text_layout(text),
            text_origin: Point::default(),
            vertical_alignment: VerticalAlignment::Middle,
        }
    }

    ///
    fn layout_text(&mut self) {
        let text_size = self.text_layout.size();

        // Adjust the text layout size to the given constraints.
        self.rectangle = self.rectangle.with_size(text_size.clamp(
            *self.size_constraints.minimum(),
            *self.size_constraints.maximum(),
        ));

        // Determine the text's horizontal position.
        let text_x = match self.horizontal_alignment {
            HorizontalAlignment::Center => {
                self.rectangle.origin().x
                    + 0.5 * (self.rectangle.size().width - text_size.width).max(0.0)
            }
            HorizontalAlignment::Left => self.rectangle.origin().x,
            HorizontalAlignment::Right => {
                self.rectangle.origin().x + self.rectangle.size().width - text_size.width
            }
        };

        // Determine the text's vertical position.
        let text_y = match self.vertical_alignment {
            VerticalAlignment::Bottom => {
                self.rectangle.origin().y
                    + (self.rectangle.size().height - text_size.height).max(0.0)
            }
            VerticalAlignment::Middle => {
                self.rectangle.origin().y
                    + 0.5 * (self.rectangle.size().height - text_size.height).max(0.0)
            }
            VerticalAlignment::Top => self.rectangle.origin().y,
        };

        // Set the text's origin.
        self.text_origin = (text_x, text_y).into();
    }
}

impl Widget for Text {
    ///
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.size_constraints = size_constraints;

        self.layout_text();

        self.rectangle.size()
    }

    fn handle_command(&mut self, widget_command: WidgetCommand) -> Result<(), WidgetError> {
        match widget_command {
            WidgetCommand::AppendChild(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.core.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::RemoveAllChildren => {
                return Err(WidgetError::CommandNotHandled(
                    self.core.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::RemoveChild(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.core.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetDebugRendering(debug_rendering) => {
                self.core.debug_rendering = debug_rendering;
            }
            WidgetCommand::SetFill(_) => {
                // TODO
                println!("`Text::handle_command(SetFill)`: TODO");
            }
            WidgetCommand::SetFont(font) => {
                self.font = font;
                self.text_layout = self.font.text_layout(self.text.clone());

                self.layout_text();
            }
            WidgetCommand::SetHasFocus(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.core.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetHorizontalAlignment(horizontal_alignment) => {
                self.horizontal_alignment = horizontal_alignment;

                // Layout.
                self.layout_text();
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
                self.layout_text();
            }
            WidgetCommand::SetVerticalAlignment(vertical_alignment) => {
                self.vertical_alignment = vertical_alignment;

                // Layout.
                self.layout_text();
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: &Event, widget_events: &mut Vec<WidgetEvent>) {
        match event {
            Event::ClipboardPaste(_) => {}
            Event::KeyDown(_) => {}
            Event::KeyUp(_) => {}
            Event::MouseDown(mouse_event) => {
                // The click is outside of the text.
                if !self.rectangle.contains(mouse_event.pos) {
                    return;
                }

                widget_events.push(WidgetEvent::Clicked(self.core.widget_id));
            }
            Event::MouseMove(_mouse_event) => {}
            Event::MouseUp(_mouse_event) => {}
        }
    }

    fn paint(&self, piet: &mut Piet, _region: &Region) -> Result<(), piet::Error> {
        // The text widget is hidden.
        if self.is_hidden {
            return Ok(());
        }

        // TODO: Check the region.

        // Draw the text clipped.
        {
            piet.save()?;
            piet.clip(&self.rectangle);
            piet.draw_text(&self.text_layout, self.text_origin);
            piet.restore()?;
        }

        // Render debug hints.
        if self.core.debug_rendering {
            piet.stroke(
                self.rectangle,
                &self.core.debug_rendering_stroke.stroke_brush,
                self.core.debug_rendering_stroke.stroke_width,
            );
        }

        Ok(())
    }

    fn rectangle(&self) -> &Rect {
        &self.rectangle
    }

    fn set_origin(&mut self, origin: Point) {
        let delta = origin - self.rectangle.origin();

        self.rectangle = self.rectangle.with_origin(origin);

        self.text_origin += delta;
    }

    fn widget_id(&self) -> &WidgetId {
        &self.core.widget_id
    }
}

// =================================================================================================

#[cfg(test)]
mod tests {
    use crate::widget::Text;
    use crate::{Font, SizeConstraints, Stroke, Widget};

    #[test]
    fn test_apply_size_constraints() {
        // Create the text widget.
        let font = Font::default();
        let mut text_widget = Text::new(0, Stroke::default(), font.clone(), "Test text");

        // Apply an unbounded `SizeConstraints`.
        {
            text_widget.apply_size_constraints(SizeConstraints::unbounded());

            assert!(
                text_widget.rectangle().size().height >= font.font_size,
                "The text widget's height should be at least as large as the font size"
            );
        }

        // Common tests are in the integration test directory.
    }

    #[test]
    fn test_handle_command() {
        // TODO
    }

    #[test]
    fn test_handle_event() {
        // TODO
    }
}
