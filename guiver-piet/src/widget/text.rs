use crate::font::Font;
use crate::shared_state::PietSharedState;
use crate::stroke::Stroke;
use crate::widget::widget_core::WidgetCore;
use crate::{Command, Event, PietWidget};
use druid_shell::piet::{PaintBrush, Piet, PietText, PietTextLayout, RenderContext, TextLayout};
use druid_shell::{kurbo, piet, Region};
use guiver::{
    HorizontalAlignment, Point, Rectangle, Size, SizeConstraints, VerticalAlignment, Widget,
    WidgetError, WidgetEvent, WidgetEventType, WidgetId, WidgetIdProvider,
};
use std::any::Any;

/// A text widget.
pub struct Text<APP_EVENT: Clone> {
    core: WidgetCore<APP_EVENT>,
    font: Font,
    horizontal_alignment: HorizontalAlignment,
    text: String,
    text_layout: PietTextLayout,
    text_origin: Point,
    vertical_alignment: VerticalAlignment,
}

impl<APP_EVENT: Clone> Text<APP_EVENT> {
    ///
    pub fn new(
        widget_id: WidgetId,
        debug_rendering_stroke: Stroke,
        piet_text: &mut PietText,
        font: Font,
        text: String,
    ) -> Self {
        Text {
            core: WidgetCore::new(widget_id, debug_rendering_stroke),
            font: font.clone(),
            horizontal_alignment: HorizontalAlignment::Center,
            text: text.clone(),
            text_layout: font.text_layout(piet_text, text),
            text_origin: Point::default(),
            vertical_alignment: VerticalAlignment::Middle,
        }
    }

    ///
    fn layout_text(&mut self) {
        let text_size = self.text_layout.size();

        {
            let maximum = *self.core.size_constraints.minimum();
            let minimum = *self.core.size_constraints.maximum();

            let size = text_size.clamp(
                kurbo::Size::new(maximum.width, maximum.height),
                kurbo::Size::new(minimum.width, minimum.height),
            );

            // Adjust the text layout size to the given constraints.
            self.core.rectangle = self
                .core
                .rectangle
                .with_size(Size::new(size.width, size.height));
        }

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
        self.text_origin = Point::new(text_x, text_y);
    }
}

impl<APP_EVENT: Clone> Widget<APP_EVENT> for Text<APP_EVENT> {
    fn add_event_observation(
        &mut self,
        widget_event_type: WidgetEventType,
        widget_event: WidgetEvent<APP_EVENT>,
    ) {
        self.core
            .add_event_observation(widget_event_type, widget_event);
    }

    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.core.size_constraints = size_constraints;

        self.layout_text();

        let size = self.core.rectangle.size();

        Size::new(size.width, size.height)
    }

    fn event_observation(
        &mut self,
        widget_event_type: &WidgetEventType,
    ) -> Option<&WidgetEvent<APP_EVENT>> {
        self.core.event_observation(widget_event_type)
    }

    fn rectangle(&self) -> &Rectangle {
        &self.core.rectangle
    }

    fn remove_event_observation(&mut self, widget_event_type: &WidgetEventType) {
        self.core.remove_event_observation(widget_event_type);
    }

    fn selected_value(&self) -> Option<Box<dyn Any>> {
        // TODO
        Some(Box::new(self.text.clone()))
    }

    fn set_debug_rendering(&mut self, debug_rendering: bool) {
        self.core.debug_rendering = debug_rendering;
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

impl<APP_EVENT: Clone> PietWidget<APP_EVENT> for Text<APP_EVENT> {
    fn handle_event(
        &mut self,
        event: &Event,
        _shared_state: &mut PietSharedState,
        _widget_id_provider: &mut WidgetIdProvider,
        widget_events: &mut Vec<WidgetEvent<APP_EVENT>>,
    ) {
        if let Event::MouseDown(mouse_event) = event {
            // The click is outside of the text.
            if !self
                .core
                .rectangle
                .contains(mouse_event.pos.x, mouse_event.pos.y)
            {
                return;
            }

            // There is a widget event observation.
            if let Some(widget_event) = self.core.event_observation(&WidgetEventType::Clicked) {
                widget_events.push(widget_event.clone());
            }
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
            piet.clip(kurbo::Rect::new(
                self.core.rectangle.x0,
                self.core.rectangle.y0,
                self.core.rectangle.x1,
                self.core.rectangle.y1,
            ));
            piet.draw_text(
                &self.text_layout,
                kurbo::Point::new(self.text_origin.x, self.text_origin.y),
            );
            piet.restore()?;
        }

        // Render debug hints.
        if self.core.debug_rendering {
            piet.stroke(
                kurbo::Rect::new(
                    self.core.rectangle.x0,
                    self.core.rectangle.y0,
                    self.core.rectangle.x1,
                    self.core.rectangle.y1,
                ),
                &self.core.debug_rendering_stroke.stroke_brush,
                self.core.debug_rendering_stroke.stroke_width,
            );
        }

        Ok(())
    }

    fn set_fill(&mut self, _fill: Option<PaintBrush>) -> Result<(), WidgetError> {
        // TODO
        println!("`Text::set_fill()`: TODO");

        Ok(())
    }

    fn set_font(
        &mut self,
        _font: Font,
        _shared_state: &mut PietSharedState,
    ) -> Result<(), WidgetError> {
        self.font = _font;
        self.text_layout = self
            .font
            .text_layout(_shared_state.piet_text(), self.text.clone());

        self.layout_text();

        Ok(())
    }

    fn set_stroke(&mut self, _stroke: Option<Stroke>) -> Result<(), WidgetError> {
        // TODO
        println!("`Text::set_stroke()`: TODO");

        Ok(())
    }

    fn set_value(
        &mut self,
        _value: Box<dyn Any>,
        _shared_state: &mut PietSharedState,
        _widget_id_provider: &mut WidgetIdProvider,
        _commands: &mut Vec<Command<APP_EVENT>>,
    ) -> Result<(), WidgetError> {
        // The given value is a string.
        if let Some(string) = _value.downcast_ref::<String>() {
            self.text = string.clone();
        }
        // The given value is something else.
        else {
            self.text = format!("{:?}", _value);
        }

        self.text_layout = self
            .font
            .text_layout(_shared_state.piet_text(), self.text.clone());
        self.layout_text();

        Ok(())
    }
}

// =================================================================================================

#[cfg(test)]
mod tests {
    use crate::font::Font;
    use crate::shared_state::piet_text;
    use crate::stroke::Stroke;
    use crate::widget::Text;
    use druid_shell::piet::PietText;
    use guiver::{SizeConstraints, Widget};

    #[test]
    fn test_apply_size_constraints() {
        // Create the text widget.
        let font = Font::default();
        let mut piet_text: PietText = piet_text();
        let mut text_widget: Text<()> = Text::new(
            0,
            Stroke::default(),
            &mut piet_text,
            font.clone(),
            "Test text".to_string(),
        );

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
