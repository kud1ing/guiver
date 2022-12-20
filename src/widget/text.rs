use crate::font::Font;
use crate::stroke::Stroke;
use crate::widget::core::WidgetCore;
use crate::widget::{WidgetError, WidgetEventType, WidgetId};
use crate::{
    Event, HorizontalAlignment, PaintBrush, SizeConstraints, VerticalAlignment, Widget, WidgetEvent,
};
use druid_shell::kurbo::{Point, Rect, Size};
use druid_shell::piet::{Piet, PietTextLayout, RenderContext, TextLayout};
use druid_shell::{piet, Region};
use std::any::Any;

/// A text widget.
pub struct Text {
    core: WidgetCore,
    font: Font,
    horizontal_alignment: HorizontalAlignment,
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
        self.core.rectangle = self.core.rectangle.with_size(text_size.clamp(
            *self.core.size_constraints.minimum(),
            *self.core.size_constraints.maximum(),
        ));

        // Determine the text's horizontal position.
        let text_x = match self.horizontal_alignment {
            HorizontalAlignment::Center => {
                self.core.rectangle.origin().x
                    + 0.5 * (self.core.rectangle.size().width - text_size.width).max(0.0)
            }
            HorizontalAlignment::Left => self.core.rectangle.origin().x,
            HorizontalAlignment::Right => {
                self.core.rectangle.origin().x + self.core.rectangle.size().width - text_size.width
            }
        };

        // Determine the text's vertical position.
        let text_y = match self.vertical_alignment {
            VerticalAlignment::Bottom => {
                self.core.rectangle.origin().y
                    + (self.core.rectangle.size().height - text_size.height).max(0.0)
            }
            VerticalAlignment::Middle => {
                self.core.rectangle.origin().y
                    + 0.5 * (self.core.rectangle.size().height - text_size.height).max(0.0)
            }
            VerticalAlignment::Top => self.core.rectangle.origin().y,
        };

        // Set the text's origin.
        self.text_origin = (text_x, text_y).into();
    }
}

impl Widget for Text {
    ///
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.core.size_constraints = size_constraints;

        self.layout_text();

        self.core.rectangle.size()
    }

    fn handle_event(&mut self, event: &Event, widget_events: &mut Vec<WidgetEvent>) {
        if let Event::MouseDown(mouse_event) = event {
            // The click is outside of the text.
            if !self.core.rectangle.contains(mouse_event.pos) {
                return;
            }

            widget_events.push((self.core.widget_id, WidgetEventType::Clicked));
        }
    }

    fn paint(&self, piet: &mut Piet, _region: &Region) -> Result<(), piet::Error> {
        // The text widget is hidden.
        if self.core.is_hidden {
            return Ok(());
        }

        // TODO: Check the region.

        // Draw the text clipped.
        {
            piet.save()?;
            piet.clip(self.core.rectangle);
            piet.draw_text(&self.text_layout, self.text_origin);
            piet.restore()?;
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

    fn selected_value(&self) -> Option<Box<dyn Any>> {
        // TODO
        Some(Box::new(self.text.clone()))
    }

    fn set_debug_rendering(&mut self, debug_rendering: bool) {
        self.core.debug_rendering = debug_rendering;
    }

    fn set_fill(&mut self, _fill: Option<PaintBrush>) -> Result<(), WidgetError> {
        // TODO
        println!("`Text::set_fill()`: TODO");

        Ok(())
    }

    fn set_font(&mut self, font: Font) -> Result<(), WidgetError> {
        self.font = font;

        // TODO: How to update the font without recreating the text layout?
        self.text_layout = self.font.text_layout(self.text.clone());

        self.layout_text();

        Ok(())
    }

    fn set_horizontal_alignment(
        &mut self,
        horizontal_alignment: HorizontalAlignment,
    ) -> Result<(), WidgetError> {
        self.horizontal_alignment = horizontal_alignment;

        // Layout.
        self.layout_text();

        Ok(())
    }

    fn set_is_disabled(&mut self, _is_disabled: bool) {
        // TODO
        println!("`Text::set_is_disabled()`: TODO");
    }

    fn set_is_hidden(&mut self, is_hidden: bool) {
        self.core.is_hidden = is_hidden;
    }

    fn set_stroke(&mut self, _stroke: Option<Stroke>) -> Result<(), WidgetError> {
        // TODO
        println!("`Text::set_stroke()`: TODO");

        Ok(())
    }

    fn set_value(&mut self, value: Box<dyn Any>) -> Result<(), WidgetError> {
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

        Ok(())
    }

    fn set_origin(&mut self, origin: Point) {
        let delta = origin - self.core.rectangle.origin();

        self.core.rectangle = self.core.rectangle.with_origin(origin);

        self.text_origin += delta;
    }

    fn set_vertical_alignment(
        &mut self,
        vertical_alignment: VerticalAlignment,
    ) -> Result<(), WidgetError> {
        self.vertical_alignment = vertical_alignment;

        // Layout.
        self.layout_text();

        Ok(())
    }

    fn value(&self) -> Option<Box<dyn Any>> {
        Some(Box::new(self.text.clone()))
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
