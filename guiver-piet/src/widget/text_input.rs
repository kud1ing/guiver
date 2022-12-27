use crate::shared_state::SharedState;
use crate::widget::core::WidgetCore;
use crate::widget::Text;
use crate::{Event, Widget};
use druid_shell::kurbo::{Line, Point, Rect, RoundedRect, Size};
use druid_shell::piet::{Color, Error, PaintBrush, Piet, PietText, RenderContext};
use druid_shell::{KbKey, Region};
use guiver::stroke::Stroke;
use guiver::{
    Font, HorizontalAlignment, SizeConstraints, VerticalAlignment, WidgetError, WidgetEvent,
    WidgetEventType, WidgetId,
};
use std::any::Any;
use std::borrow::BorrowMut;

/// A text input widget.
pub struct TextInput {
    caret_character_index: usize,
    caret_x: f64,
    caret_y1: f64,
    caret_y2: f64,
    core: WidgetCore,
    corner_radius: f64,
    fill: Option<PaintBrush>,
    has_focus: bool,
    horizontal_alignment: HorizontalAlignment,
    is_disabled: bool,
    padding: f64,
    stroke: Stroke,
    stroke_focused: Stroke,
    text: String,
    text_widget: Text,
    width: f64,
}

impl TextInput {
    ///
    pub fn new(
        widget_id: WidgetId,
        debug_rendering_stroke: Stroke,
        piet_text: &mut PietText,
        font: Font,
        text: impl Into<String>,
        width: f64,
        frame_color: Color,
        frame_color_focused: Color,
    ) -> Self {
        let child_widget_id = 0;
        let text = text.into();

        TextInput {
            caret_character_index: 0,
            caret_x: 0.0,
            caret_y1: 0.0,
            caret_y2: 0.0,
            core: WidgetCore::new(widget_id, debug_rendering_stroke.clone()),
            corner_radius: 2.0,
            fill: None,
            has_focus: false,
            horizontal_alignment: HorizontalAlignment::Center,
            is_disabled: true,
            padding: 4.0,
            stroke: Stroke {
                stroke_brush: PaintBrush::Color(frame_color),
                stroke_style: Default::default(),
                stroke_width: 1.0,
            },
            stroke_focused: Stroke {
                stroke_brush: PaintBrush::Color(frame_color_focused),
                stroke_style: Default::default(),
                stroke_width: 1.0,
            },
            text: text.clone(),
            text_widget: Text::new(
                child_widget_id,
                debug_rendering_stroke,
                piet_text,
                font,
                text,
            ),
            width,
        }
    }

    ///
    fn broadcast_modified_text(
        &mut self,
        shared_state: &mut SharedState,
        widget_events: &mut Vec<WidgetEvent>,
    ) {
        // Pass the updated text to the child text widget.
        self.update_text_widget(shared_state);

        // Inform the world that the text has changed.
        widget_events.push((self.core.widget_id, WidgetEventType::ValueChanged));
    }

    ///
    fn layout_child_widget(&mut self) {
        // Add the padding to the border size.
        let mut border_width = 2.0 * self.padding;
        let mut border_height = 2.0 * self.padding;

        // Add the stroke width to the border size.
        {
            let stroke = self.stroke();

            border_width += 2.0 * stroke.stroke_width;
            border_height += 2.0 * stroke.stroke_width;
        }

        // Apply the child widget's size constraints.
        let child_size = self.text_widget.borrow_mut().apply_size_constraints(
            self.core
                .size_constraints
                .shrink(Size::new(border_width, border_height)),
        );

        self.core.rectangle = self.core.rectangle.with_size(
            Size::new(self.width + border_width, child_size.height + border_height).clamp(
                *self.core.size_constraints.minimum(),
                *self.core.size_constraints.maximum(),
            ),
        );

        // Set the child widget's origin.
        {
            let child_origin = {
                let delta_child_x = match self.horizontal_alignment {
                    HorizontalAlignment::Center => {
                        0.5 * (self.core.rectangle.size().width - child_size.width).max(0.0)
                    }
                    HorizontalAlignment::Left => self.padding,
                    HorizontalAlignment::Right => {
                        (self.core.rectangle.size().width - child_size.width).max(0.0)
                            - self.padding
                    }
                };
                let delta_child_y =
                    0.5 * (self.core.rectangle.size().height - child_size.height).max(0.0);

                self.core.rectangle.origin() + (delta_child_x, delta_child_y)
            };

            self.text_widget.borrow_mut().set_origin(child_origin);
        }

        // Update the caret postion and dimension.
        self.update_caret_position();
    }

    ///
    fn update_caret_character_index(&mut self) {
        // TODO: Decrease the index when the text shrank.

        // Update the caret postion and dimension.
        self.update_caret_position();
    }

    ///
    fn update_caret_position(&mut self) {
        // Put the care to the right of the text widget.
        self.caret_x = self.text_widget.rectangle().x1;

        self.caret_y1 = self.core.rectangle.y0 + self.padding;
        self.caret_y2 = self.core.rectangle.y1 - self.padding;
    }

    ///
    fn stroke(&self) -> &Stroke {
        // Stroke.
        if self.has_focus {
            &self.stroke_focused
        } else {
            &self.stroke
        }
    }

    ///
    fn update_text_widget(&mut self, shared_state: &mut SharedState) {
        // Pass the updated text to the child text widget.
        self.text_widget
            .borrow_mut()
            .set_value(shared_state, Box::new(self.text.clone()))
            .unwrap();

        // Update the caret index, if necessary.
        self.update_caret_character_index();

        self.layout_child_widget();
    }
}

impl Widget for TextInput {
    ///
    fn accepts_focus(&self) -> bool {
        true
    }

    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.core.size_constraints = size_constraints;

        // Layout the child widget.
        self.layout_child_widget();

        self.core.rectangle.size()
    }

    fn handle_event(
        &mut self,
        shared_state: &mut SharedState,
        event: &Event,
        widget_events: &mut Vec<WidgetEvent>,
    ) {
        match event {
            Event::ClipboardPaste(string) => {
                // TODO: error handling
                self.set_selected_value(shared_state, Box::new(string.clone()))
                    .unwrap();
            }
            Event::KeyDown(key_event) => match &key_event.key {
                KbKey::Character(chracter_string) => {
                    // Append the character to the text.
                    self.text.push_str(chracter_string);

                    // Apply the text changes.
                    self.broadcast_modified_text(shared_state, widget_events);
                }
                KbKey::Backspace => {
                    if !self.text.is_empty() {
                        // Drop the last character from the text.
                        let mut character_iterator = self.text.chars();
                        character_iterator.next_back();
                        self.text = character_iterator.as_str().to_string();
                    }

                    // Apply the text changes.
                    self.broadcast_modified_text(shared_state, widget_events);
                }
                KbKey::Enter => {
                    // Enter on a (focused) text input submits the value.
                    widget_events.push((self.core.widget_id, WidgetEventType::Submitted));
                }
                _ => {}
            },
            Event::MouseDown(mouse_event) => {
                // The mouse is down within this text input.
                if self.core.rectangle.contains(mouse_event.pos) {
                    // This widget has no focus.
                    if !self.has_focus {
                        // Accept focus.
                        self.has_focus = true;

                        // Tell the widget manager about the change of focus.
                        widget_events.push((self.core.widget_id, WidgetEventType::GainedFocus))
                    }
                }
                // The mouse is down outside of this text input.
                else {
                    // This widget has focus.
                    if self.has_focus {
                        // Give up focus.
                        self.has_focus = false;

                        // Tell the widget manager about the change of focus.
                        widget_events.push((self.core.widget_id, WidgetEventType::LostFocus));
                    }
                }
            }
            _ => {}
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), Error> {
        // The text input widget is hidden.
        if self.core.is_hidden {
            return Ok(());
        }

        let stroke = self.stroke();

        // Paint the frame.
        {
            let shape = RoundedRect::from_rect(self.core.rectangle, self.corner_radius);

            // Fill the frame.
            if let Some(fill_brush) = &self.fill {
                piet.fill(shape, fill_brush);
            }

            // Stroke the frame.
            piet.stroke(shape, &stroke.stroke_brush, stroke.stroke_width);
        }

        // Paint the text widget.
        self.text_widget.paint(piet, region)?;

        // Draw the caret.
        if self.has_focus {
            piet.stroke_styled(
                Line::new((self.caret_x, self.caret_y1), (self.caret_x, self.caret_y2)),
                &stroke.stroke_brush,
                1.0,
                &stroke.stroke_style,
            );
        }

        // Render debug hints.
        if self.core.debug_rendering {
            piet.stroke(
                self.core.rectangle,
                &self.core.debug_rendering_stroke.stroke_brush,
                self.core.debug_rendering_stroke.stroke_width,
            );
        }

        Ok(())
    }

    fn rectangle(&self) -> &Rect {
        &self.core.rectangle
    }

    fn remove_selected_value(&mut self, shared_state: &mut SharedState) -> Result<(), WidgetError> {
        // TODO
        println!("`TextInput::remove_selected_value()`: TODO");
        self.set_value(shared_state, Box::new("".to_string()))
    }

    fn selected_value(&self) -> Option<Box<dyn Any>> {
        self.text_widget.selected_value()
    }

    fn set_debug_rendering(&mut self, debug_rendering: bool) {
        self.core.debug_rendering = debug_rendering;
    }

    fn set_fill(&mut self, fill: Option<PaintBrush>) -> Result<(), WidgetError> {
        self.fill = fill;
        Ok(())
    }

    fn set_font(&mut self, shared_state: &mut SharedState, font: Font) -> Result<(), WidgetError> {
        self.text_widget.set_font(shared_state, font)
    }

    fn set_has_focus(&mut self, has_focus: bool) -> Result<(), WidgetError> {
        self.has_focus = has_focus;
        Ok(())
    }

    fn set_horizontal_alignment(
        &mut self,
        horizontal_alignment: HorizontalAlignment,
    ) -> Result<(), WidgetError> {
        self.horizontal_alignment = horizontal_alignment;

        Ok(())
    }

    fn set_is_disabled(&mut self, is_disabled: bool) {
        self.is_disabled = is_disabled;
    }

    fn set_is_hidden(&mut self, is_hidden: bool) {
        self.core.is_hidden = is_hidden;
    }

    fn set_origin(&mut self, origin: Point) {
        self.core.rectangle = self.core.rectangle.with_origin(origin);

        // Layout the child widget.
        self.layout_child_widget();
    }

    fn set_stroke(&mut self, _stroke: Option<Stroke>) -> Result<(), WidgetError> {
        // TODO
        println!("`TextInput::set_stroke()`: TODO");
        Ok(())
    }

    fn set_selected_value(
        &mut self,
        shared_state: &mut SharedState,
        value: Box<dyn Any>,
    ) -> Result<(), WidgetError> {
        // TODO
        println!("`TextInput::set_selected_value()`: TODO");
        self.set_value(shared_state, value)
    }

    fn set_value(
        &mut self,
        shared_state: &mut SharedState,
        value: Box<dyn Any>,
    ) -> Result<(), WidgetError> {
        // The given value is a string.
        if let Some(string) = value.downcast_ref::<String>() {
            self.text = string.clone();

            // Apply the text changes.
            self.update_text_widget(shared_state);
        }

        Ok(())
    }

    fn set_vertical_alignment(
        &mut self,
        _vertical_alignment: VerticalAlignment,
    ) -> Result<(), WidgetError> {
        // TODO
        println!("`TextInput::set_vertical_alignment()`: TODO");

        Ok(())
    }

    fn value(&self) -> Option<Box<dyn Any>> {
        self.text_widget.value()
    }

    fn widget_id(&self) -> &WidgetId {
        &self.core.widget_id
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