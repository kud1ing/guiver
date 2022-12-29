use crate::shared_state::PietSharedState;
use crate::widget::core::WidgetCore;

use crate::widget_manager::PietWidgetBox;
use crate::{Event, PietWidget};
use druid_shell::kurbo::{Point, Rect, RoundedRect, Size};
use druid_shell::piet::{Color, LinearGradient, PaintBrush, Piet, RenderContext, UnitPoint};
use druid_shell::{piet, KbKey, Region};
use guiver::stroke::Stroke;
use guiver::{
    Font, HorizontalAlignment, SizeConstraints, VerticalAlignment, Widget, WidgetError,
    WidgetEvent, WidgetEventType, WidgetId, WidgetIdProvider, WidgetPlacement,
};
use std::any::Any;

///
#[derive(Default)]
pub struct Button {
    child_widget: Option<PietWidgetBox>,
    core: WidgetCore,
    corner_radius: f64,
    fill_brush_down: Option<PaintBrush>,
    fill_brush_up: Option<PaintBrush>,
    has_focus: bool,
    is_disabled: bool,
    is_down: bool,
    is_hot: bool,
    padding_horizontal: f64,
    padding_vertical: f64,
    stroke: Option<Stroke>,
    stroke_focused: Option<Stroke>,
}

impl Button {
    ///
    pub fn new(
        widget_id: WidgetId,
        debug_rendering_stroke: Stroke,
        child_widget: PietWidgetBox,
        fill_brush_down: Option<PaintBrush>,
        frame_color: Option<Color>,
        frame_color_focused: Option<Color>,
    ) -> Self {
        Button {
            child_widget: Some(child_widget),
            core: WidgetCore::new(widget_id, debug_rendering_stroke),
            corner_radius: 4.0,
            fill_brush_down,
            fill_brush_up: Some(PaintBrush::Linear(LinearGradient::new(
                UnitPoint::TOP,
                UnitPoint::BOTTOM,
                (Color::rgb8(100, 100, 100), Color::rgb8(50, 50, 50)),
            ))),
            has_focus: false,
            is_disabled: false,
            is_down: false,
            is_hot: false,
            padding_horizontal: 4.0,
            padding_vertical: 4.0,
            stroke: frame_color.map(|color| Stroke {
                stroke_brush: PaintBrush::Color(color),
                stroke_style: Default::default(),
                stroke_width: 1.0,
            }),
            stroke_focused: frame_color_focused.map(|color| Stroke {
                stroke_brush: PaintBrush::Color(color),
                stroke_style: Default::default(),
                stroke_width: 1.0,
            }),
        }
    }

    ///
    fn layout_child_widget(&mut self) {
        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            let padding_size =
                Size::new(2.0 * self.padding_horizontal, 2.0 * self.padding_vertical);

            // Apply the child widget's size constraints.
            let child_size = child_widget
                .borrow_mut()
                .apply_size_constraints(self.core.size_constraints.shrink(padding_size));

            self.core.rectangle = self
                .core
                .rectangle
                .with_size((child_size + padding_size).clamp(
                    *self.core.size_constraints.minimum(),
                    *self.core.size_constraints.maximum(),
                ));

            // Set the child widget's origin.
            {
                let child_origin = self.core.rectangle.origin()
                    + (
                        0.5 * (self.core.rectangle.size().width - child_size.width).max(0.0),
                        0.5 * (self.core.rectangle.size().height - child_size.height).max(0.0),
                    );

                // Set the child widget's origin.
                child_widget.borrow_mut().set_origin(child_origin);
            }
        }
        // There is no child widget.
        else {
            self.core.rectangle = self
                .core
                .rectangle
                .with_size(*self.core.size_constraints.minimum());
        }
    }
}

impl Widget for Button {
    fn accepts_focus(&self) -> bool {
        true
    }

    fn widget_id(&self) -> &WidgetId {
        &self.core.widget_id
    }
}

impl PietWidget for Button {
    fn add_child(
        &mut self,
        _widget_placement: Option<WidgetPlacement>,
        child_widget: PietWidgetBox,
    ) -> Result<(), WidgetError> {
        // TODO: use `_widget_placement`?

        self.child_widget = Some(child_widget.clone());

        // Layout the child widget.
        self.layout_child_widget();

        Ok(())
    }

    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.core.size_constraints = size_constraints;

        // Layout the child widget.
        self.layout_child_widget();

        self.core.rectangle.size()
    }

    fn handle_event(
        &mut self,
        _widget_id_provider: &mut WidgetIdProvider,
        _shared_state: &mut PietSharedState,
        event: &Event,
    ) -> Vec<WidgetEvent> {
        let mut widget_events = vec![];

        match event {
            Event::KeyDown(key_event) => {
                if key_event.key == KbKey::Enter {
                    // Enter on a (focused) button is like a click.
                    widget_events.push((self.core.widget_id, WidgetEventType::Clicked));
                }
            }
            Event::MouseDown(mouse_event) => {
                // The mouse is down within this button.
                if self.core.rectangle.contains(mouse_event.pos) {
                    // This widget was not focused.
                    if !self.has_focus {
                        // Give it focus.
                        self.has_focus = true;

                        // Tell the widget manager about the change of focus.
                        widget_events.push((self.core.widget_id, WidgetEventType::GainedFocus))
                    }

                    self.is_down = true;
                    self.is_hot = true;
                }
                // The mouse is down outside of this button.
                else {
                    // This widget was focused.
                    if self.has_focus {
                        // Tell the widget manager about the change of focus.
                        widget_events.push((self.core.widget_id, WidgetEventType::LostFocus))
                    }

                    self.has_focus = false;
                    self.is_down = false;
                    self.is_hot = false;
                }
            }
            Event::MouseMove(mouse_event) => {
                // The mouse moved inside of this button.
                if self.is_hot && self.core.rectangle.contains(mouse_event.pos) {
                    self.is_down = true;
                }
                // The mouse moved outside of this button.
                else {
                    self.is_down = false;
                }
            }
            Event::MouseUp(mouse_event) => {
                if self.is_hot && self.core.rectangle.contains(mouse_event.pos) {
                    widget_events.push((self.core.widget_id, WidgetEventType::Clicked));
                }

                self.is_down = false;
                self.is_hot = false;
            }
            _ => {}
        }

        widget_events
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), piet::Error> {
        // The button widget is hidden.
        if self.core.is_hidden {
            return Ok(());
        }

        // Paint the button itself.
        {
            let button_shape = RoundedRect::from_rect(self.core.rectangle, self.corner_radius);

            if self.is_down {
                if let Some(brush) = &self.fill_brush_down {
                    piet.fill(button_shape, brush);
                }
            } else if let Some(brush) = &self.fill_brush_up {
                piet.fill(button_shape, brush);
            }

            // Stroke.
            // The button is focused.
            if self.has_focus {
                // There is a focuse stroke brush.
                if let Some(stroke) = &self.stroke_focused {
                    piet.stroke(button_shape, &stroke.stroke_brush, stroke.stroke_width);
                }
            }
            // The button is not focused.
            else {
                // There is a stroke brush.
                if let Some(stroke) = &self.stroke {
                    piet.stroke(button_shape, &stroke.stroke_brush, stroke.stroke_width);
                }
            }
        }

        // There is a child widget.
        if let Some(child_widget_rc) = &self.child_widget {
            // Paint the child widget.
            child_widget_rc.borrow().paint(piet, region)?;
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

    fn remove_children(&mut self) -> Result<(), WidgetError> {
        self.child_widget = None;
        Ok(())
    }

    fn set_debug_rendering(&mut self, debug_rendering: bool) {
        self.core.debug_rendering = debug_rendering;
    }

    fn set_fill(&mut self, fill: Option<PaintBrush>) -> Result<(), WidgetError> {
        self.fill_brush_up = fill;
        Ok(())
    }

    fn set_font(
        &mut self,
        shared_state: &mut PietSharedState,
        font: Font,
    ) -> Result<(), WidgetError> {
        if let Some(child_widget) = &mut self.child_widget {
            child_widget.borrow_mut().set_font(shared_state, font)?;
        }

        Ok(())
    }

    fn set_has_focus(&mut self, has_focus: bool) -> Result<(), WidgetError> {
        self.has_focus = has_focus;
        Ok(())
    }

    fn set_horizontal_alignment(
        &mut self,
        _horizontal_alignment: HorizontalAlignment,
    ) -> Result<(), WidgetError> {
        // TODO
        println!("`Button::set_horizontal_alignment()`: TODO");

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

    fn set_stroke(&mut self, stroke: Option<Stroke>) -> Result<(), WidgetError> {
        self.stroke = stroke;
        Ok(())
    }

    fn set_value(
        &mut self,
        _widget_id_provider: &mut WidgetIdProvider,
        _shared_state: &mut PietSharedState,
        _value: Box<dyn Any>,
    ) -> Result<(), WidgetError> {
        // TODO
        println!("`Button::set_value()`: TODO");

        Ok(())
    }

    fn set_vertical_alignment(
        &mut self,
        _vertical_alignment: VerticalAlignment,
    ) -> Result<(), WidgetError> {
        // TODO
        println!("`Button::set_vertical_alignment()`: TODO");

        Ok(())
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
