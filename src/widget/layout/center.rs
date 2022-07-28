use crate::stroke::Stroke;
use crate::widget::{WidgetCommand, WidgetError, WidgetId};
use crate::widget_manager::WidgetBox;
use crate::{SizeConstraints, SystemEvent, Widget, WidgetEvent};
use druid_shell::kurbo::{Point, Rect, Size};
use druid_shell::piet::{Piet, RenderContext};
use druid_shell::{piet, Region};

/// A centering layout widget.
pub struct Center {
    child_widget: Option<WidgetBox>,
    debug_rendering: bool,
    debug_rendering_stroke: Stroke,
    is_hidden: bool,
    rectangle: Rect,
    size_constraints: SizeConstraints,
    widget_id: WidgetId,
}

impl Center {
    ///
    pub fn new(widget_id: WidgetId, debug_rendering_stroke: Stroke) -> Self {
        Center {
            child_widget: None,
            debug_rendering: false,
            debug_rendering_stroke,
            is_hidden: false,
            rectangle: Rect::default(),
            size_constraints: SizeConstraints::default(),
            widget_id,
        }
    }

    ///
    fn layout_child(&mut self) {
        self.rectangle = self.rectangle.with_size(*self.size_constraints.maximum());

        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            // Apply the child widget's size constraints.
            let child_size = child_widget
                .borrow_mut()
                .apply_size_constraints(self.size_constraints);

            // Set the child's origin.
            child_widget.borrow_mut().set_origin(
                self.rectangle.origin()
                    + (
                        0.5 * (self.rectangle.size().width - child_size.width).max(0.0),
                        0.5 * (self.rectangle.size().height - child_size.height).max(0.0),
                    ),
            );
        }
    }
}

impl Widget for Center {
    ///
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
                println!("`Center::handle_widget_command(RemoveChild)`: TODO");
            }
            WidgetCommand::SetDebugRendering(debug_rendering) => {
                self.debug_rendering = debug_rendering;
            }
            WidgetCommand::SetHasFocus(_) => {}
            WidgetCommand::SetIsDisabled(_) => {
                // TODO
                println!("`Center::handle_widget_command(SetIsDisabled)`: TODO");
            }
            WidgetCommand::SetIsHidden(is_hidden) => {
                self.is_hidden = is_hidden;
            }
            WidgetCommand::SetValue(_value) => {}
        }

        Ok(())
    }

    fn handle_event(&mut self, system_event: &SystemEvent, widget_events: &mut Vec<WidgetEvent>) {
        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            child_widget
                .borrow_mut()
                .handle_event(system_event, widget_events);
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), piet::Error> {
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

    fn widget_id(&self) -> &WidgetId {
        &self.widget_id
    }
}
