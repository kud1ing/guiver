use crate::widget::{WidgetCommand, WidgetId};
use crate::{SizeConstraints, UserEvent, Widget, WidgetEvent};
use druid_shell::kurbo::{Point, Size};
use druid_shell::piet::Piet;
use druid_shell::Region;
use std::collections::HashMap;

/// A padding layout widget.
pub struct Padding {
    child_widget: Option<Box<dyn Widget>>,
    is_hidden: bool,
    origin: Point,
    padding_bottom: f64,
    padding_left: f64,
    padding_right: f64,
    padding_top: f64,
    size: Size,
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
        child_widget: Option<Box<dyn Widget>>,
    ) -> Self {
        Padding {
            child_widget,
            is_hidden: false,
            origin: (0.0, 0.0).into(),
            padding_bottom,
            padding_left,
            padding_right,
            padding_top,
            size: Size::default(),
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
                child_widget.apply_size_constraints(self.size_constraints.shrink(Size::new(
                    self.padding_left + self.padding_right,
                    self.padding_top + self.padding_bottom,
                )));

            // Set the children's origins
            child_widget.set_origin(
                self.origin
                    + (
                        0.5 * (self.size.width - child_size.width).max(0.0),
                        0.5 * (self.size.height - child_size.height).max(0.0),
                    ),
            );
        }
    }

    ///
    pub fn set_child_widget(&mut self, child_widget: Option<Box<dyn Widget>>) {
        self.child_widget = child_widget;

        // Layout the child.
        self.layout_child();
    }
}

impl Widget for Padding {
    ///
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.size_constraints = size_constraints;
        self.size = *size_constraints.maximum();

        // Layout the child.
        self.layout_child();

        self.size
    }

    fn handle_commands(
        &mut self,
        widget_command_dictionary: &HashMap<WidgetId, Vec<WidgetCommand>>,
    ) {
        // There are commands for this widget.
        if let Some(widget_commands) = widget_command_dictionary.get(&self.widget_id) {
            for widget_command in widget_commands {
                match widget_command {
                    WidgetCommand::Clear => {
                        self.child_widget = None;
                    }
                    WidgetCommand::Remove => {
                        // A widget can not remove itself.
                    }
                    WidgetCommand::SetHasFocus(_) => {}
                    WidgetCommand::SetIsDisabled(_) => {
                        // TODO
                        println!("`Padding::handle_widget_command(SetIsDisabled)`: TODO");
                    }
                    WidgetCommand::SetIsHidden(is_hidden) => {
                        // Hide/show this widget.
                        self.set_is_hidden(*is_hidden);
                    }
                    WidgetCommand::SetValue(_value) => {}
                }
            }
        }

        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            // Let the child widget handle the remaining commands.
            child_widget.handle_commands(widget_command_dictionary);
        }
    }

    fn handle_event(&mut self, event: &UserEvent, widget_events: &mut Vec<WidgetEvent>) {
        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            child_widget.handle_event(event, widget_events);
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) {
        if self.is_hidden {
            return;
        }

        // There is a child widget.
        if let Some(child_widget_rc) = &self.child_widget {
            // Paint the child widget.
            child_widget_rc.paint(piet, region);
        }
    }

    fn set_has_focus(&mut self, _has_focus: bool) {
        // Nothing to do.
    }

    fn set_is_hidden(&mut self, is_hidden: bool) {
        self.is_hidden = is_hidden;

        // Hide/show the child widget.
        if let Some(child_widget) = &mut self.child_widget {
            child_widget.set_is_hidden(is_hidden);
        }
    }

    fn set_origin(&mut self, origin: Point) {
        self.origin = origin;

        // Layout the child.
        self.layout_child();
    }

    fn widget_id(&self) -> &WidgetId {
        &self.widget_id
    }
}
