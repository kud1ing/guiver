use crate::widget::{WidgetCommand, WidgetError};
use crate::widget_manager::WidgetBox;
use crate::{Event, Piet, Size, SizeConstraints, Stroke, Widget, WidgetEvent, WidgetId};
use druid_shell::kurbo::{Point, Rect};
use druid_shell::piet::{Error, RenderContext};
use druid_shell::Region;

/// A widget that takes all of the available space.
pub struct Expanded {
    child_widget: Option<WidgetBox>,
    debug_rendering: bool,
    debug_rendering_stroke: Stroke,
    flex_factor: u16,
    is_hidden: bool,
    rectangle: Rect,
    size_constraints: SizeConstraints,
    widget_id: WidgetId,
}

impl Expanded {
    ///
    pub fn new(widget_id: WidgetId, debug_rendering_stroke: Stroke, flex_factor: u16) -> Self {
        Expanded {
            child_widget: None,
            debug_rendering: false,
            debug_rendering_stroke,
            flex_factor,
            is_hidden: false,
            rectangle: Rect::default(),
            size_constraints: SizeConstraints::unbounded(),
            widget_id,
        }
    }

    ///
    fn layout_child(&mut self) {
        self.rectangle = self.rectangle.with_size(*self.size_constraints.maximum());

        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            // Set the child widget's size.
            child_widget
                .borrow_mut()
                .apply_size_constraints(SizeConstraints::tight(self.rectangle.size()));

            // Set the child widget's origin.
            child_widget
                .borrow_mut()
                .set_origin(self.rectangle.origin());
        }
    }
}

impl Widget for Expanded {
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.size_constraints = size_constraints;

        // Layout the child.
        self.layout_child();

        self.rectangle.size()
    }

    fn flex_factor(&self) -> u16 {
        self.flex_factor
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
                    println!("`Expanded::handle_command(RemoveChild)`: TODO");
                }
                // There is no child widget.
                else {
                    return Err(WidgetError::CommandNotHandled(
                        self.widget_id,
                        widget_command,
                    ));
                }
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
                println!("`Expanded::handle_command(SetIsDisabled)`: TODO");
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
            child_widget.borrow_mut().handle_event(event, widget_events);
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), Error> {
        // The expanded widget is hidden.
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

    fn size(&self) -> Size {
        self.rectangle.size()
    }

    fn widget_id(&self) -> &WidgetId {
        &self.widget_id
    }
}
