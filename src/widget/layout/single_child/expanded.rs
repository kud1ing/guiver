use crate::widget::core::WidgetCore;
use crate::widget::{WidgetCommand, WidgetError};
use crate::widget_manager::WidgetBox;
use crate::{Event, Piet, Size, SizeConstraints, Stroke, Widget, WidgetEvent, WidgetId};
use druid_shell::kurbo::{Point, Rect};
use druid_shell::piet::{Error, RenderContext};
use druid_shell::Region;

/// A layout widget that tries to adjust its child widget to take all of the available space.
/// Mostly useful in `Column` and `Row`.
pub struct Expanded {
    child_widget: Option<WidgetBox>,
    core: WidgetCore,
    flex_factor: u16,
}

impl Expanded {
    ///
    pub fn new(widget_id: WidgetId, debug_rendering_stroke: Stroke, flex_factor: u16) -> Self {
        Expanded {
            child_widget: None,
            core: WidgetCore::new(widget_id, debug_rendering_stroke),
            flex_factor,
        }
    }

    ///
    fn layout_child_widget(&mut self) {
        self.core.rectangle = self
            .core
            .rectangle
            .with_size(*self.core.size_constraints.maximum());

        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            // Set the child widget's size.
            child_widget
                .borrow_mut()
                .apply_size_constraints(SizeConstraints::tight(self.core.rectangle.size()));

            // Set the child widget's origin.
            child_widget
                .borrow_mut()
                .set_origin(self.core.rectangle.origin());
        }
    }
}

impl Widget for Expanded {
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.core.size_constraints = size_constraints;

        // Layout the child.
        self.layout_child_widget();

        self.core.rectangle.size()
    }

    fn flex_factor(&self) -> u16 {
        self.flex_factor
    }

    fn handle_command(&mut self, widget_command: &WidgetCommand) -> Result<(), WidgetError> {
        match widget_command {
            WidgetCommand::AddChild(_widget_placement, child_widget) => {
                self.child_widget = Some(child_widget.clone());

                // Layout the child.
                self.layout_child_widget();

                return Ok(());
            }
            WidgetCommand::RemoveAllChildren => {
                self.child_widget = None;

                // Update this widget's size.
                self.layout_child_widget();

                return Ok(());
            }
            WidgetCommand::RemoveChild(child_widget_id) => {
                // There is a child widget.
                if let Some(_child_widget) = &mut self.child_widget {
                    // TODO
                    println!("`Expanded::handle_command(RemoveChild)`: TODO");

                    // Update this widget's size.
                    self.layout_child_widget();

                    return Ok(());
                }
                // There is no child widget.
                else {
                    return Err(WidgetError::NoSuchWidget(*child_widget_id));
                }
            }
            _ => {}
        }

        self.core.handle_command(widget_command)
    }

    fn handle_event(&mut self, event: &Event, widget_events: &mut Vec<WidgetEvent>) {
        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            child_widget.borrow_mut().handle_event(event, widget_events);
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), Error> {
        // The expanded widget is hidden.
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
        self.layout_child_widget();
    }

    fn widget_id(&self) -> &WidgetId {
        &self.core.widget_id
    }
}
