use crate::widget::{WidgetCommand, WidgetError, WidgetId};
use crate::{SizeConstraints, SystemEvent, Widget, WidgetEvent};
use druid_shell::kurbo::{Point, Size};
use druid_shell::piet::Piet;
use druid_shell::Region;
use std::cmp::max;

///
pub struct Row {
    child_widgets: Vec<Box<dyn Widget>>,
    is_hidden: bool,
    origin: Point,
    size: Size,
    size_constraints: SizeConstraints,
    spacing: f64,
    widget_id: WidgetId,
}

impl Row {
    ///
    pub fn new(widget_id: WidgetId, spacing: f64, child_widgets: Vec<Box<dyn Widget>>) -> Self {
        Row {
            child_widgets,
            is_hidden: false,
            origin: (0.0, 0.0).into(),
            size: Size::default(),
            size_constraints: SizeConstraints::unbounded(),
            spacing,
            widget_id,
        }
    }

    ///
    pub fn append_child_widget(&mut self, child_widget: Box<dyn Widget>) {
        self.child_widgets.push(child_widget);

        // Layout the children.
        self.layout_children();
    }

    ///
    fn layout_children(&mut self) {
        let number_of_child_widgets = self.child_widgets.len();
        let number_of_spacers = max(number_of_child_widgets - 1, 0);

        // TODO: improve the layout algorithm used here.

        let child_size_constraints = SizeConstraints::new(
            Size::new(0.0, self.size.height),
            Size::new(
                (self.size.width - number_of_spacers as f64 * self.spacing)
                    / (number_of_child_widgets as f64),
                self.size.height,
            ),
        );

        let mut x = self.origin.x;

        for child_widget in &mut self.child_widgets {
            let child_size = child_widget.apply_size_constraints(child_size_constraints);

            // Set the children's origins.
            child_widget.set_origin((x, self.origin.y).into());

            x += child_size.width + self.spacing;
        }
    }
}

impl Widget for Row {
    ///
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.size_constraints = size_constraints;
        self.size = *size_constraints.maximum();

        // Layout the children.
        self.layout_children();

        self.size
    }

    fn handle_commands(&mut self, widget_commands: &[WidgetCommand]) -> Result<(), WidgetError> {
        for widget_command in widget_commands {
            match widget_command {
                WidgetCommand::Clear => {
                    self.child_widgets.clear();
                }
                WidgetCommand::RemoveChild(_) => {
                    // TODO
                    println!("`Row::handle_widget_command(RemoveChild)`: TODO");
                }
                WidgetCommand::SetHasFocus(_) => {}
                WidgetCommand::SetIsDisabled(_) => {
                    // TODO
                    println!("`Row::handle_widget_command(SetIsDisabled)`: TODO");
                }
                WidgetCommand::SetIsHidden(is_hidden) => {
                    // Hide/show this widget.
                    self.is_hidden = *is_hidden;
                }
                WidgetCommand::SetValue(_) => {}
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, system_event: &SystemEvent, widget_events: &mut Vec<WidgetEvent>) {
        // Iterate over the child widgets.
        for child_widget in &mut self.child_widgets {
            child_widget.handle_event(system_event, widget_events);
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) {
        if self.is_hidden {
            return;
        }

        // Iterate over the child widgets.
        for current_child_widget in &self.child_widgets {
            // Paint the current child widget.
            current_child_widget.paint(piet, region);
        }
    }

    fn set_origin(&mut self, origin: Point) {
        self.origin = origin;

        // Layout the children.
        self.layout_children();
    }

    fn widget_id(&self) -> &WidgetId {
        &self.widget_id
    }
}
