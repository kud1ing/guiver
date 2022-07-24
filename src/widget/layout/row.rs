use crate::widget::{WidgetCommand, WidgetError, WidgetId};
use crate::widget_manager::WidgetBox;
use crate::{SizeConstraints, SystemEvent, Widget, WidgetEvent};
use druid_shell::kurbo::{Point, Rect, Size};
use druid_shell::piet::Piet;
use druid_shell::Region;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::cmp::max;

///
pub struct Row {
    child_widgets: Vec<WidgetBox>,
    is_hidden: bool,
    rectangle: Rect,
    size_constraints: SizeConstraints,
    spacing: f64,
    widget_id: WidgetId,
}

impl Row {
    ///
    pub fn new(widget_id: WidgetId, spacing: f64) -> Self {
        Row {
            child_widgets: vec![],
            is_hidden: false,
            rectangle: Rect::default(),
            size_constraints: SizeConstraints::unbounded(),
            spacing,
            widget_id,
        }
    }

    ///
    fn layout_children(&mut self) {
        let number_of_child_widgets = self.child_widgets.len();

        // There are no children.
        if number_of_child_widgets <= 0 {
            return;
        }

        let number_of_spacers = max(number_of_child_widgets - 1, 0);

        // TODO: improve the layout algorithm used here.

        let child_size_constraints = SizeConstraints::new(
            Size::new(0.0, self.rectangle.size().height),
            Size::new(
                (self.rectangle.size().width - number_of_spacers as f64 * self.spacing)
                    / (number_of_child_widgets as f64),
                self.rectangle.size().height,
            ),
        );

        let mut child_x = self.rectangle.origin().x;

        for child_widget in &mut self.child_widgets {
            let child_size = RefCell::borrow_mut(&child_widget)
                .borrow_mut()
                .apply_size_constraints(child_size_constraints);

            // Set the children's origins.
            RefCell::borrow_mut(&child_widget)
                .borrow_mut()
                .set_origin((child_x, self.rectangle.origin().y).into());

            child_x += child_size.width + self.spacing;
        }
    }
}

impl Widget for Row {
    ///
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.size_constraints = size_constraints;
        self.rectangle = self.rectangle.with_size(*size_constraints.maximum());

        // Layout the children.
        self.layout_children();

        self.rectangle.size()
    }

    fn handle_command(&mut self, widget_command: WidgetCommand) -> Result<(), WidgetError> {
        match widget_command {
            WidgetCommand::AppendChild(child_widget) => {
                self.child_widgets.push(child_widget);
            }
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
                self.is_hidden = is_hidden;
            }
            WidgetCommand::SetValue(_) => {}
        }

        Ok(())
    }

    fn handle_event(&mut self, system_event: &SystemEvent, widget_events: &mut Vec<WidgetEvent>) {
        // Iterate over the child widgets.
        for child_widget in &mut self.child_widgets {
            RefCell::borrow_mut(&child_widget).handle_event(system_event, widget_events);
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) {
        if self.is_hidden {
            return;
        }

        // Iterate over the child widgets.
        for child_widget in &self.child_widgets {
            RefCell::borrow(&child_widget).paint(piet, region);
        }
    }

    fn set_origin(&mut self, origin: Point) {
        self.rectangle = self.rectangle.with_origin(origin);

        // Layout the children.
        self.layout_children();
    }

    fn widget_id(&self) -> &WidgetId {
        &self.widget_id
    }
}
