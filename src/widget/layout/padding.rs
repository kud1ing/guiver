use crate::widget::{WidgetCommand, WidgetError, WidgetId};
use crate::widget_manager::WidgetBox;
use crate::{SizeConstraints, SystemEvent, Widget, WidgetEvent};
use druid_shell::kurbo::{Point, Rect, Size};
use druid_shell::piet::Piet;
use druid_shell::Region;

/// A padding layout widget.
pub struct Padding {
    child_widget: Option<WidgetBox>,
    is_hidden: bool,
    padding_bottom: f64,
    padding_left: f64,
    padding_right: f64,
    padding_top: f64,
    rectangle: Rect,
    size_constraints: SizeConstraints,
    widget_id: WidgetId,
}

impl Padding {
    ///
    pub fn new(
        widget_id: WidgetId,
        padding_left: f64,
        padding_top: f64,
        padding_right: f64,
        padding_bottom: f64,
    ) -> Self {
        Padding {
            child_widget: None,
            is_hidden: false,
            padding_bottom,
            padding_left,
            padding_right,
            padding_top,
            rectangle: Rect::default(),
            size_constraints: SizeConstraints::unbounded(),
            widget_id,
        }
    }

    ///
    fn layout_child(&mut self) {
        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            // Apply the child widget's size constraints.
            let child_size =
                child_widget
                    .borrow_mut()
                    .apply_size_constraints(self.size_constraints.shrink(Size::new(
                        self.padding_left + self.padding_right,
                        self.padding_top + self.padding_bottom,
                    )));

            // Set the children's origins
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

impl Widget for Padding {
    ///
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.size_constraints = size_constraints;
        self.rectangle = self.rectangle.with_size(*size_constraints.maximum());

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
            WidgetCommand::Clear => {
                self.child_widget = None;
            }
            WidgetCommand::RemoveChild(_) => {
                // TODO
                println!("`Padding::handle_widget_command(RemoveChild)`: TODO");
            }
            WidgetCommand::SetHasFocus(_) => {}
            WidgetCommand::SetIsDisabled(_) => {
                // TODO
                println!("`Padding::handle_widget_command(SetIsDisabled)`: TODO");
            }
            WidgetCommand::SetIsHidden(is_hidden) => {
                // Hide/show this widget.
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

    fn paint(&self, piet: &mut Piet, region: &Region) {
        if self.is_hidden {
            return;
        }

        // There is a child widget.
        if let Some(child_widget_rc) = &self.child_widget {
            // Paint the child widget.
            child_widget_rc.borrow().paint(piet, region);
        }
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
