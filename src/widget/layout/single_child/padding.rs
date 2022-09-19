use crate::stroke::Stroke;
use crate::widget::core::WidgetCore;
use crate::widget::{WidgetCommand, WidgetError, WidgetId};
use crate::widget_manager::WidgetBox;
use crate::{Event, SizeConstraints, Widget, WidgetEvent};
use druid_shell::kurbo::{Point, Rect, Size};
use druid_shell::piet::{Piet, RenderContext};
use druid_shell::{piet, Region};

/// A padding layout widget.
pub struct Padding {
    child_widget: Option<WidgetBox>,
    core: WidgetCore,
    is_hidden: bool,
    padding_bottom: f64,
    padding_left: f64,
    padding_right: f64,
    padding_top: f64,
    rectangle: Rect,
    size_constraints: SizeConstraints,
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
            is_hidden: false,
            padding_bottom,
            padding_left,
            padding_right,
            padding_top,
            rectangle: Rect::default(),
            size_constraints: SizeConstraints::default(),
        }
    }

    ///
    fn layout_child(&mut self) {
        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            let padding_size = Size::new(
                self.padding_left + self.padding_right,
                self.padding_top + self.padding_bottom,
            );

            // Apply the child widget's size constraints.
            let child_size = child_widget
                .borrow_mut()
                .apply_size_constraints(self.size_constraints.shrink(padding_size));

            self.rectangle = self.rectangle.with_size(child_size + padding_size);

            // Set the child's origin.
            child_widget.borrow_mut().set_origin(
                self.rectangle.origin()
                    + (
                        0.5 * (self.rectangle.size().width - child_size.width).max(0.0),
                        0.5 * (self.rectangle.size().height - child_size.height).max(0.0),
                    ),
            );
        }
        // There is no child widget.
        else {
            self.rectangle = self.rectangle.with_size(*self.size_constraints.maximum());
        }
    }
}

impl Widget for Padding {
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
                // There is a child widget.
                if let Some(_child_widget) = &mut self.child_widget {
                    // TODO
                    println!("`Padding::handle_command(RemoveChild)`: TODO");
                }
                // There is no child widget.
                else {
                    return Err(WidgetError::CommandNotHandled(
                        self.core.widget_id,
                        widget_command,
                    ));
                }
            }
            WidgetCommand::SetDebugRendering(debug_rendering) => {
                self.core.debug_rendering = debug_rendering;
            }
            WidgetCommand::SetFill(ref _value) => {
                return Err(WidgetError::CommandNotHandled(
                    self.core.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetFont(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.core.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetHasFocus(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.core.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetHorizontalAlignment(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.core.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetIsDisabled(_) => {
                // TODO
                println!("`Padding::handle_command(SetIsDisabled)`: TODO");
            }
            WidgetCommand::SetIsHidden(is_hidden) => {
                self.is_hidden = is_hidden;
            }
            WidgetCommand::SetStroke(ref _value) => {
                return Err(WidgetError::CommandNotHandled(
                    self.core.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetValue(ref _value) => {
                return Err(WidgetError::CommandNotHandled(
                    self.core.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetVerticalAlignment(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.core.widget_id,
                    widget_command,
                ));
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: &Event, widget_events: &mut Vec<WidgetEvent>) {
        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            child_widget.borrow_mut().handle_event(event, widget_events);
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), piet::Error> {
        // The padding widget is hidden.
        if self.is_hidden {
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
                self.rectangle,
                &self.core.debug_rendering_stroke.stroke_brush,
                self.core.debug_rendering_stroke.stroke_width,
            );
        }

        Ok(())
    }

    fn rectangle(&self) -> &Rect {
        &self.rectangle
    }

    fn set_origin(&mut self, origin: Point) {
        self.rectangle = self.rectangle.with_origin(origin);

        // Layout the child.
        self.layout_child();
    }

    fn widget_id(&self) -> &WidgetId {
        &self.core.widget_id
    }
}
