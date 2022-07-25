use crate::widget::{Text, WidgetCommand, WidgetError};
use crate::{SizeConstraints, SystemEvent, Widget, WidgetEvent, WidgetId};
use druid_shell::kurbo::{Point, Rect, RoundedRect, Size};
use druid_shell::piet::{Color, Error, PaintBrush, Piet, RenderContext};
use druid_shell::{KbKey, Region};
use std::borrow::BorrowMut;

/// A text input widget.
pub struct TextInput {
    corner_radius: f64,
    debug_rendering: bool,
    debug_rendering_stroke_brush: PaintBrush,
    debug_rendering_stroke_width: f64,
    fill_brush: Option<PaintBrush>,
    has_focus: bool,
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
        debug_rendering_stroke_brush: PaintBrush,
        debug_rendering_stroke_width: f64,
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
            debug_rendering_stroke_brush: debug_rendering_stroke_brush.clone(),
            debug_rendering_stroke_width: debug_rendering_stroke_width.clone(),
            fill_brush: None,
            has_focus: false,
            is_disabled: true,
            is_hidden: false,
            padding: 4.0,
            rectangle: Rect::default(),
            size_constraints: SizeConstraints::unbounded(),
            stroke_brush: PaintBrush::Color(frame_color),
            stroke_brush_focused: PaintBrush::Color(frame_color_focused),
            stroke_width: 1.0,
            text: text.clone(),
            text_widget: Text::new(
                child_widget_id,
                debug_rendering_stroke_brush,
                debug_rendering_stroke_width,
                text,
            ),
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
        self.text_widget.borrow_mut().set_origin(
            self.rectangle.origin()
                + (
                    0.5 * (self.rectangle.size().width - child_size.width).max(0.0),
                    0.5 * (self.rectangle.size().height - child_size.height).max(0.0),
                ),
        );
    }

    ///
    fn update_text_widget(&mut self) {
        // Pass the updated text to the child text widget.
        self.text_widget
            .borrow_mut()
            .handle_command(WidgetCommand::SetValue(Box::new(self.text.clone())))
            .unwrap();
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
            WidgetCommand::SetHasFocus(has_focus) => {
                self.has_focus = has_focus;
            }
            WidgetCommand::SetIsDisabled(is_disabled) => {
                self.is_disabled = is_disabled;
            }
            WidgetCommand::SetIsHidden(is_hidden) => {
                self.is_hidden = is_hidden;
            }
            WidgetCommand::SetValue(value) => {
                // The given value is a string.
                if let Some(string) = value.downcast_ref::<String>() {
                    self.text = string.clone();

                    // Apply the text changes.
                    self.update_text_widget();
                }
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, system_event: &SystemEvent, widget_events: &mut Vec<WidgetEvent>) {
        match system_event {
            SystemEvent::KeyDown(key_event) => match &key_event.key {
                KbKey::Character(string) => {
                    // Append the character to the text.
                    self.text.push_str(&string);

                    // Apply the text changes.
                    self.broadcast_modified_text(widget_events);
                }
                KbKey::Backspace => {
                    if self.text.len() > 0 {
                        // Drop the last character from the text.
                        self.text.remove(self.text.len() - 1);
                    }

                    // Apply the text changes.
                    self.broadcast_modified_text(widget_events);
                }
                _ => {}
            },
            SystemEvent::KeyUp(_key_event) => {}
            SystemEvent::MouseDown(mouse_event) => {
                // The mouse is down within this text input.
                if self.rectangle.contains(mouse_event.pos) {
                    // This widget was not focused.
                    if !self.has_focus {
                        // Give it focus.
                        self.has_focus = true;

                        // Tell the widget manager about the change of focus.
                        widget_events.push(WidgetEvent::GotFocus(self.widget_id))
                    }
                }
                // The mouse is down outside of this text input.
                else {
                    // This widget was focused.
                    if self.has_focus {
                        // Tell the widget manager about the change of focus.
                        widget_events.push(WidgetEvent::LostFocus(self.widget_id))
                    }

                    self.has_focus = false;
                }
            }
            SystemEvent::MouseMove(_) => {}
            SystemEvent::MouseUp(_) => {}
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), Error> {
        // Paint the input field frame.
        {
            let shape = RoundedRect::from_rect(self.rectangle, self.corner_radius);

            // Fill.
            if let Some(fill_brush) = &self.fill_brush {
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
                &self.debug_rendering_stroke_brush,
                self.debug_rendering_stroke_width,
            );
        }

        Ok(())
    }

    fn set_origin(&mut self, origin: Point) {
        self.rectangle = self.rectangle.with_origin(origin);

        // Layout the child.
        self.layout_child();
    }

    fn widget_id(&self) -> &WidgetId {
        &self.widget_id
    }
}
