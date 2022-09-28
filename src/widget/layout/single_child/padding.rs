use crate::stroke::Stroke;
use crate::widget::core::WidgetCore;
use crate::widget::{WidgetCommand, WidgetError, WidgetId};
use crate::widget_manager::WidgetBox;
use crate::{Event, SizeConstraints, Widget, WidgetEvent};
use druid_shell::kurbo::{Point, Rect, Size};
use druid_shell::piet::{Piet, RenderContext};
use druid_shell::{piet, Region};

/// A layout widget that adds padding around its child widget.
pub struct Padding {
    child_widget: Option<WidgetBox>,
    core: WidgetCore,
    padding_bottom: f64,
    padding_left: f64,
    padding_right: f64,
    padding_top: f64,
}

impl Padding {
    ///
    pub fn new(
        widget_id: WidgetId,
        debug_rendering_stroke: Stroke,
        padding_left: f64,
        padding_top: f64,
        padding_right: f64,
        padding_bottom: f64,
    ) -> Self {
        Padding {
            child_widget: None,
            core: WidgetCore::new(widget_id, debug_rendering_stroke),
            padding_bottom,
            padding_left,
            padding_right,
            padding_top,
        }
    }

    ///
    fn layout_child_widget(&mut self) {
        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            let padding_size = Size::new(
                self.padding_left + self.padding_right,
                self.padding_top + self.padding_bottom,
            );

            // Apply the child widget's size constraints.
            let child_size = child_widget
                .borrow_mut()
                .apply_size_constraints(self.core.size_constraints.shrink(padding_size));

            self.core.rectangle = self.core.rectangle.with_size(child_size + padding_size);

            // Set the child widget's origin.
            child_widget.borrow_mut().set_origin(
                self.core.rectangle.origin()
                    + (
                        0.5 * (self.core.rectangle.size().width - child_size.width).max(0.0),
                        0.5 * (self.core.rectangle.size().height - child_size.height).max(0.0),
                    ),
            );
        }
        // There is no child widget.
        else {
            self.core.rectangle = self
                .core
                .rectangle
                .with_size(*self.core.size_constraints.maximum());
        }
    }
}

impl Widget for Padding {
    ///
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.core.size_constraints = size_constraints;

        // Layout the child widget.
        self.layout_child_widget();

        self.core.rectangle.size()
    }

    fn handle_command(&mut self, widget_command: &WidgetCommand) -> Result<(), WidgetError> {
        match widget_command {
            WidgetCommand::AddChild(_widget_placement, child_widget) => {
                self.child_widget = Some(child_widget.clone());

                // Layout the child widget.
                self.layout_child_widget();

                Ok(())
            }
            WidgetCommand::RemoveAllChildren => {
                self.child_widget = None;

                // Update this widget's size.
                self.layout_child_widget();

                Ok(())
            }
            WidgetCommand::RemoveChild(_) => {
                // There is a child widget.
                if let Some(_child_widget) = &mut self.child_widget {
                    // TODO
                    println!("`Padding::handle_command(RemoveChild)`: TODO");

                    // Update this widget's size.
                    self.layout_child_widget();

                    Ok(())
                }
                // There is no child widget.
                else {
                    Err(WidgetError::CommandNotHandled(
                        self.core.widget_id,
                        format!("{:?}", widget_command),
                    ))
                }
            }
            _ => self.core.handle_command(widget_command),
        }
    }

    fn handle_event(&mut self, event: &Event, widget_events: &mut Vec<WidgetEvent>) {
        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            child_widget.borrow_mut().handle_event(event, widget_events);
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), piet::Error> {
        // The padding widget is hidden.
        if self.core.is_hidden {
            return Ok(());
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

    fn set_origin(&mut self, origin: Point) {
        self.core.rectangle = self.core.rectangle.with_origin(origin);

        // Layout the child widget.
        self.layout_child_widget();
    }

    fn widget_id(&self) -> &WidgetId {
        &self.core.widget_id
    }
}
