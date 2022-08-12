use crate::widget::{WidgetCommand, WidgetError};
use crate::widget_manager::WidgetBox;
use crate::{Event, Piet, SizeConstraints, Stroke, Widget, WidgetEvent, WidgetId};
use druid_shell::kurbo::{Point, Rect, Size};
use druid_shell::piet::{Error, RenderContext};
use druid_shell::Region;

///
pub struct SizedBox {
    child_widget: Option<WidgetBox>,
    debug_rendering: bool,
    debug_rendering_stroke: Stroke,
    desired_size: Size,
    is_hidden: bool,
    rectangle: Rect,
    size_constraints: SizeConstraints,
    widget_id: WidgetId,
}

impl SizedBox {
    ///
    pub fn new(widget_id: WidgetId, debug_rendering_stroke: Stroke, desired_size: Size) -> Self {
        SizedBox {
            child_widget: None,
            debug_rendering: false,
            debug_rendering_stroke,
            desired_size,
            is_hidden: false,
            rectangle: Rect::default(),
            size_constraints: SizeConstraints::default(),
            widget_id,
        }
    }

    ///
    fn layout_child(&mut self) {
        let widget_size = self.desired_size.clamp(
            *self.size_constraints.minimum(),
            *self.size_constraints.maximum(),
        );

        self.rectangle = self.rectangle.with_size(widget_size);

        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            // Apply the child widget's size constraints.
            child_widget
                .borrow_mut()
                .apply_size_constraints(SizeConstraints::tight(widget_size));

            // Set the child's origin.
            child_widget
                .borrow_mut()
                .set_origin(self.rectangle.origin());
        }
    }
}

impl Widget for SizedBox {
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
                println!("`SizedBox::handle_command(RemoveChild)`: TODO");
            }
            WidgetCommand::SetDebugRendering(debug_rendering) => {
                self.debug_rendering = debug_rendering;
            }
            WidgetCommand::SetFill(ref _value) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetFont(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetHasFocus(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetHorizontalAlignment(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetIsDisabled(_) => {
                // TODO
                println!("`SizedBox::handle_command(SetIsDisabled)`: TODO");
            }
            WidgetCommand::SetIsHidden(is_hidden) => {
                self.is_hidden = is_hidden;
            }
            WidgetCommand::SetStroke(ref _value) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetValue(ref _value) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetVerticalAlignment(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
        }

        Ok(())
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
        if self.is_hidden {
            return Ok(());
        }

        // There is a child widget.
        if let Some(child_widget_rc) = &self.child_widget {
            // Paint the child widget.
            child_widget_rc.borrow().paint(piet, region)?;
        }

        // Render debug hints.
        if self.debug_rendering {
            piet.stroke(
                self.rectangle,
                &self.debug_rendering_stroke.stroke_brush,
                self.debug_rendering_stroke.stroke_width,
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
