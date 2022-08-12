use crate::stroke::Stroke;
use crate::widget::{Text, WidgetCommand, WidgetError};
use crate::{Event, Font, HorizontalAlignment, SizeConstraints, Widget, WidgetEvent, WidgetId};
use druid_shell::kurbo::{Point, Rect, RoundedRect, Size};
use druid_shell::piet::{Color, Error, PaintBrush, Piet, RenderContext};
use druid_shell::{KbKey, Region};
use std::borrow::BorrowMut;

/// A text input widget.
pub struct TextInput {
    corner_radius: f64,
    debug_rendering: bool,
    debug_rendering_stroke: Stroke,
    fill: Option<PaintBrush>,
    has_focus: bool,
    horizontal_alignment: HorizontalAlignment,
    is_disabled: bool,
    is_hidden: bool,
    padding: f64,
    rectangle: Rect,
    size_constraints: SizeConstraints,
    stroke_brush: PaintBrush,
    stroke_brush_focused: PaintBrush,
    stroke_width: f64,
    text: String,
    text_widget: Text,
    widget_id: WidgetId,
    width: f64,
}

impl TextInput {
    ///
    pub fn new(
        widget_id: WidgetId,
        debug_rendering_stroke: Stroke,
        font: Font,
        text: impl Into<String>,
        width: f64,
        frame_color: Color,
        frame_color_focused: Color,
    ) -> Self {
        let child_widget_id = 0;
        let text = text.into();

        TextInput {
            corner_radius: 2.0,
            debug_rendering: false,
            debug_rendering_stroke: debug_rendering_stroke.clone(),
            fill: None,
            has_focus: false,
            horizontal_alignment: HorizontalAlignment::Center,
            is_disabled: true,
            is_hidden: false,
            padding: 4.0,
            rectangle: Rect::default(),
            size_constraints: SizeConstraints::unbounded(),
            stroke_brush: PaintBrush::Color(frame_color),
            stroke_brush_focused: PaintBrush::Color(frame_color_focused),
            stroke_width: 1.0,
            text: text.clone(),
            text_widget: Text::new(child_widget_id, debug_rendering_stroke, font, text),
            widget_id,
            width,
        }
    }

    ///
    fn broadcast_modified_text(&mut self, widget_events: &mut Vec<WidgetEvent>) {
        // Pass the updated text to the child text widget.
        self.update_text_widget();

        // Inform the world that the text has changed.
        widget_events.push(WidgetEvent::ValueChanged(
            self.widget_id,
            Box::new(self.text.clone()),
        ));
    }

    ///
    fn layout_child(&mut self) {
        let padding_size = Size::new(2.0 * self.padding, 2.0 * self.padding);

        // Apply the child widget's size constraints.
        let child_size = self
            .text_widget
            .borrow_mut()
            .apply_size_constraints(self.size_constraints.shrink(padding_size));

        // Make the text input at least as wide as its width.
        self.rectangle = self.rectangle.with_size(Size::new(
            self.width.max(child_size.width + padding_size.width),
            child_size.height + padding_size.height,
        ));

        // Set the child's origin.
        {
            let delta_child_x = match self.horizontal_alignment {
                HorizontalAlignment::Center => {
                    0.5 * (self.rectangle.size().width - child_size.width).max(0.0)
                }
                HorizontalAlignment::Left => self.padding,
                HorizontalAlignment::Right => {
                    (self.rectangle.size().width - child_size.width).max(0.0) - self.padding
                }
            };
            let delta_child_y = 0.5 * (self.rectangle.size().height - child_size.height).max(0.0);

            self.text_widget
                .borrow_mut()
                .set_origin(self.rectangle.origin() + (delta_child_x, delta_child_y));
        }
    }

    ///
    fn update_text_widget(&mut self) {
        // Pass the updated text to the child text widget.
        self.text_widget
            .borrow_mut()
            .handle_command(WidgetCommand::SetValue(Box::new(self.text.clone())))
            .unwrap();

        self.layout_child();
    }
}

impl Widget for TextInput {
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.size_constraints = size_constraints;

        // Layout the child.
        self.layout_child();

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
            WidgetCommand::RemoveChild(widget_id) => {
                return Err(WidgetError::NoSuchWidget(widget_id));
            }
            WidgetCommand::SetDebugRendering(debug_rendering) => {
                self.debug_rendering = debug_rendering;
            }
            WidgetCommand::SetFill(fill) => {
                self.fill = fill;
            }
            WidgetCommand::SetFont(_) => {
                self.text_widget.handle_command(widget_command)?;
            }
            WidgetCommand::SetHasFocus(has_focus) => {
                self.has_focus = has_focus;
            }
            WidgetCommand::SetHorizontalAlignment(horizontal_alignment) => {
                self.horizontal_alignment = horizontal_alignment;
            }
            WidgetCommand::SetIsDisabled(is_disabled) => {
                self.is_disabled = is_disabled;
            }
            WidgetCommand::SetIsHidden(is_hidden) => {
                self.is_hidden = is_hidden;
            }
            WidgetCommand::SetStroke(_) => {
                // TODO
                println!("`TextInput::handle_command(SetStroke)`: TODO");
            }
            WidgetCommand::SetValue(value) => {
                // The given value is a string.
                if let Some(string) = value.downcast_ref::<String>() {
                    self.text = string.clone();

                    // Apply the text changes.
                    self.update_text_widget();
                }
            }
            WidgetCommand::SetVerticalAlignment(_) => {
                // TODO
                println!("`TextInput::handle_command(SetVerticalAlignment)`: TODO");
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: &Event, widget_events: &mut Vec<WidgetEvent>) {
        match event {
            Event::KeyDown(key_event) => match &key_event.key {
                KbKey::Character(string) => {
                    // Append the character to the text.
                    self.text.push_str(string);

                    // Apply the text changes.
                    self.broadcast_modified_text(widget_events);
                }
                KbKey::Backspace => {
                    if !self.text.is_empty() {
                        // Drop the last character from the text.
                        self.text.remove(self.text.len() - 1);
                    }

                    // Apply the text changes.
                    self.broadcast_modified_text(widget_events);
                }
                KbKey::Enter => {
                    // Enter on a (focused) text input submits the value.
                    widget_events.push(WidgetEvent::ValueSubmitted(self.widget_id));
                }
                _ => {}
            },
            Event::KeyUp(_key_event) => {}
            Event::MouseDown(mouse_event) => {
                // The mouse is down within this text input.
                if self.rectangle.contains(mouse_event.pos) {
                    // This widget has no focus.
                    if !self.has_focus {
                        // Accept focus.
                        self.has_focus = true;

                        // Tell the widget manager about the change of focus.
                        widget_events.push(WidgetEvent::GainedFocus(self.widget_id))
                    }
                }
                // The mouse is down outside of this text input.
                else {
                    // This widget has focus.
                    if self.has_focus {
                        // Give up focus.
                        self.has_focus = false;

                        // Tell the widget manager about the change of focus.
                        widget_events.push(WidgetEvent::LostFocus(self.widget_id));
                    }
                }
            }
            Event::MouseMove(_) => {}
            Event::MouseUp(_) => {}
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), Error> {
        // The text input widget is hidden.
        if self.is_hidden {
            return Ok(());
        }

        // Paint the input field frame.
        {
            let shape = RoundedRect::from_rect(self.rectangle, self.corner_radius);

            // Fill.
            if let Some(fill_brush) = &self.fill {
                piet.fill(shape, fill_brush);
            }

            // Stroke.
            if self.has_focus {
                piet.stroke(shape, &self.stroke_brush_focused, self.stroke_width);
            } else {
                piet.stroke(shape, &self.stroke_brush, self.stroke_width);
            }
        }

        // Paint the inner content.
        {
            piet.save()?;
            piet.clip(&self.rectangle);

            // Paint the text.
            self.text_widget.paint(piet, region)?;

            piet.restore()?;
        }

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

        // Layout the child.
        self.layout_child();
    }

    fn size(&self) -> Size {
        self.rectangle.size()
    }

    fn widget_id(&self) -> &WidgetId {
        &self.widget_id
    }
}

// =================================================================================================

#[cfg(test)]
mod tests {

    #[test]
    fn test_apply_size_constraints() {
        // TODO: Create the widget.

        // TODO: Apply an unbounded `SizeConstraints`.

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
