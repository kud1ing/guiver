use crate::widget::core::WidgetCore;
use crate::widget::{WidgetCommand, WidgetError};
use crate::widget_manager::WidgetBox;
use crate::{Event, Piet, SizeConstraints, Stroke, Widget, WidgetEvent, WidgetId};
use druid_shell::kurbo::{Point, Rect, Size};
use druid_shell::piet::{Error, RenderContext};
use druid_shell::Region;

/// A layout widget that tries to adjust its child widget to a desired size.
pub struct SizedBox {
    child_widget: Option<WidgetBox>,
    core: WidgetCore,
    desired_size: Size,
}

impl SizedBox {
    ///
    pub fn new(widget_id: WidgetId, debug_rendering_stroke: Stroke, desired_size: Size) -> Self {
        SizedBox {
            child_widget: None,
            core: WidgetCore::new(widget_id, debug_rendering_stroke),
            desired_size,
        }
    }

    ///
    fn layout_child(&mut self) {
        let widget_size = self.desired_size.clamp(
            *self.core.size_constraints.minimum(),
            *self.core.size_constraints.maximum(),
        );

        self.core.rectangle = self.core.rectangle.with_size(widget_size);

        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            // Apply the child widget's size constraints.
            child_widget
                .borrow_mut()
                .apply_size_constraints(SizeConstraints::tight(widget_size));

            // Set the child's origin.
            child_widget
                .borrow_mut()
                .set_origin(self.core.rectangle.origin());
        }
    }
}

impl Widget for SizedBox {
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.core.size_constraints = size_constraints;

        // Layout the child.
        self.layout_child();

        self.core.rectangle.size()
    }

    fn handle_command(&mut self, widget_command: &WidgetCommand) -> Result<(), WidgetError> {
        match widget_command {
            WidgetCommand::AddChild(child_widget) => {
                self.child_widget = Some(child_widget.clone());

                // Layout the child.
                self.layout_child();

                Ok(())
            }
            WidgetCommand::RemoveAllChildren => {
                self.child_widget = None;
                Ok(())
            }
            WidgetCommand::RemoveChild(_) => {
                // TODO
                println!("`SizedBox::handle_command(RemoveChild)`: TODO");

                Ok(())
            }
            _ => self.core.handle_command(widget_command),
        }
    }

    fn handle_event(&mut self, event: &Event, widget_events: &mut Vec<WidgetEvent>) {
        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            // Let the child widget handle the event.
            child_widget.borrow_mut().handle_event(event, widget_events);
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), Error> {
        // The sized box widget is hidden.
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

        // Layout the child.
        self.layout_child();
    }

    fn widget_id(&self) -> &WidgetId {
        &self.core.widget_id
    }
}
