use crate::stroke::Stroke;
use crate::widget::{WidgetCommand, WidgetError};
use crate::widget_manager::WidgetBox;
use crate::{Event, SizeConstraints, Widget, WidgetEvent, WidgetId};
use druid_shell::kurbo::{Point, Rect, RoundedRect, Size};
use druid_shell::piet::{Color, LinearGradient, PaintBrush, Piet, RenderContext, UnitPoint};
use druid_shell::{piet, KbKey, Region};

///
pub struct Button {
    child_widget: Option<WidgetBox>,
    corner_radius: f64,
    debug_rendering: bool,
    debug_rendering_stroke: Stroke,
    fill_brush_down: Option<PaintBrush>,
    fill_brush_up: Option<PaintBrush>,
    has_focus: bool,
    is_disabled: bool,
    is_down: bool,
    is_hidden: bool,
    is_hot: bool,
    padding_horizontal: f64,
    padding_vertical: f64,
    rectangle: Rect,
    size_constraints: SizeConstraints,
    stroke: Option<Stroke>,
    stroke_focused: Option<Stroke>,
    widget_id: WidgetId,
}

impl Button {
    ///
    pub fn new(
        widget_id: WidgetId,
        debug_rendering_stroke: Stroke,
        child_widget: WidgetBox,
        fill_brush_down: Option<PaintBrush>,
        frame_color: Option<Color>,
        frame_color_focused: Option<Color>,
    ) -> Self {
        Button {
            child_widget: Some(child_widget),
            corner_radius: 4.0,
            debug_rendering: false,
            debug_rendering_stroke,
            fill_brush_down,
            fill_brush_up: Some(PaintBrush::Linear(LinearGradient::new(
                UnitPoint::TOP,
                UnitPoint::BOTTOM,
                (Color::rgb8(100, 100, 100), Color::rgb8(50, 50, 50)),
            ))),
            has_focus: false,
            is_disabled: false,
            is_down: false,
            is_hidden: false,
            is_hot: false,
            padding_horizontal: 4.0,
            padding_vertical: 4.0,
            rectangle: Rect::default(),
            size_constraints: SizeConstraints::default(),
            stroke: frame_color.map(|color| Stroke {
                brush: PaintBrush::Color(color),
                style: Default::default(),
                width: 1.0,
            }),
            stroke_focused: frame_color_focused.map(|color| Stroke {
                brush: PaintBrush::Color(color),
                style: Default::default(),
                width: 1.0,
            }),
            widget_id,
        }
    }

    ///
    fn layout_child(&mut self) {
        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            let padding_size =
                Size::new(2.0 * self.padding_horizontal, 2.0 * self.padding_vertical);

            // Apply the child widget's size constraints.
            let child_size = child_widget
                .borrow_mut()
                .apply_size_constraints(self.size_constraints.shrink(padding_size));

            // Set the child's origin.
            child_widget.borrow_mut().set_origin(
                self.rectangle.origin()
                    + (
                        0.5 * (self.rectangle.size().width - child_size.width).max(0.0),
                        0.5 * (self.rectangle.size().height - child_size.height).max(0.0),
                    ),
            );

            self.rectangle = self.rectangle.with_size(child_size + padding_size);
        }
        // There is no child widget.
        else {
            self.rectangle = self.rectangle.with_size(*self.size_constraints.maximum());
        }
    }
}

impl Widget for Button {
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.size_constraints = size_constraints;

        // Layout the child.
        self.layout_child();

        self.rectangle.size()
    }

    fn handle_command(&mut self, widget_command: WidgetCommand) -> Result<(), WidgetError> {
        match widget_command {
            WidgetCommand::AppendChild(child_widget) => {
                self.child_widget = Some(child_widget);

                // Layout the child.
                self.layout_child();
            }
            WidgetCommand::RemoveAllChildren => {
                self.child_widget = None;
            }
            WidgetCommand::RemoveChild(_) => {
                // TODO
                println!("`Button::handle_command(RemoveChild)`: TODO");
            }
            WidgetCommand::SetDebugRendering(debug_rendering) => {
                self.debug_rendering = debug_rendering;
            }
            WidgetCommand::SetFill(fill) => {
                self.fill_brush_up = fill;
            }
            WidgetCommand::SetFont(_) => {
                if let Some(child_widget) = &mut self.child_widget {
                    child_widget.borrow_mut().handle_command(widget_command)?;
                }
            }
            WidgetCommand::SetHasFocus(has_focus) => {
                self.has_focus = has_focus;
            }
            WidgetCommand::SetHorizontalAlignment(_) => {
                // TODO
                println!("`Button::handle_command(SetHorizontalAlignment)`: TODO");
            }
            WidgetCommand::SetIsDisabled(is_disabled) => {
                self.is_disabled = is_disabled;
            }
            WidgetCommand::SetIsHidden(is_hidden) => {
                self.is_hidden = is_hidden;
            }
            WidgetCommand::SetStroke(stroke) => {
                self.stroke = stroke;
            }
            WidgetCommand::SetValue(_) => {
                // TODO
                println!("`Button::handle_command(SetValue)`: TODO");
            }
            WidgetCommand::SetVerticalAlignment(_) => {
                // TODO
                println!("`Button::handle_command(SetVerticalAlignment)`: TODO");
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: &Event, widget_events: &mut Vec<WidgetEvent>) {
        match event {
            Event::KeyDown(key_event) => {
                if key_event.key == KbKey::Enter {
                    // Enter on a (focused) button is like a click.
                    widget_events.push(WidgetEvent::Clicked(self.widget_id));
                }
            }
            Event::KeyUp(_) => {}
            Event::MouseDown(mouse_event) => {
                // The mouse is down within this button.
                if self.rectangle.contains(mouse_event.pos) {
                    // This widget was not focused.
                    if !self.has_focus {
                        // Give it focus.
                        self.has_focus = true;

                        // Tell the widget manager about the change of focus.
                        widget_events.push(WidgetEvent::GainedFocus(self.widget_id))
                    }

                    self.is_down = true;
                    self.is_hot = true;
                }
                // The mouse is down outside of this button.
                else {
                    // This widget was focused.
                    if self.has_focus {
                        // Tell the widget manager about the change of focus.
                        widget_events.push(WidgetEvent::LostFocus(self.widget_id))
                    }

                    self.has_focus = false;
                    self.is_down = false;
                    self.is_hot = false;
                }
            }
            Event::MouseMove(mouse_event) => {
                // The mouse moved inside of this button.
                if self.is_hot && self.rectangle.contains(mouse_event.pos) {
                    self.is_down = true;
                }
                // The mouse moved outside of this button.
                else {
                    self.is_down = false;
                }
            }
            Event::MouseUp(mouse_event) => {
                if self.is_hot && self.rectangle.contains(mouse_event.pos) {
                    widget_events.push(WidgetEvent::Clicked(self.widget_id));
                }

                self.is_down = false;
                self.is_hot = false;
            }
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), piet::Error> {
        if self.is_hidden {
            return Ok(());
        }

        // Paint the button itself.
        {
            let button_shape = RoundedRect::from_rect(self.rectangle, self.corner_radius);

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
                    piet.stroke(button_shape, &stroke.brush, stroke.width);
                }
            }
            // The button is not focused.
            else {
                // There is a stroke brush.
                if let Some(stroke) = &self.stroke {
                    piet.stroke(button_shape, &stroke.brush, stroke.width);
                }
            }
        }

        // There is a child widget.
        if let Some(child_widget_rc) = &self.child_widget {
            // TODO: Clip the child widget.

            // Paint the child widget.
            child_widget_rc.borrow().paint(piet, region)?;
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
