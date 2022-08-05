use crate::stroke::Stroke;
use crate::widget::{WidgetCommand, WidgetError, WidgetId};
use crate::widget_manager::WidgetBox;
use crate::{Event, SizeConstraints, VerticalAlignment, Widget, WidgetEvent};
use druid_shell::kurbo::{Point, Rect, Size};
use druid_shell::piet::{Piet, RenderContext};
use druid_shell::{piet, Region};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::cmp::max;

///
pub struct Row {
    child_widgets: Vec<WidgetBox>,
    debug_rendering: bool,
    debug_rendering_stroke: Stroke,
    is_hidden: bool,
    rectangle: Rect,
    size_constraints: SizeConstraints,
    spacing: f64,
    vertical_alignment: VerticalAlignment,
    widget_id: WidgetId,
}

impl Row {
    ///
    pub fn new(
        widget_id: WidgetId,
        debug_rendering_stroke: Stroke,
        vertical_alignment: VerticalAlignment,
        spacing: f64,
    ) -> Self {
        Row {
            child_widgets: vec![],
            debug_rendering: false,
            debug_rendering_stroke,
            is_hidden: false,
            rectangle: Rect::default(),
            size_constraints: SizeConstraints::unbounded(),
            spacing,
            vertical_alignment,
            widget_id,
        }
    }

    ///
    fn layout_children(&mut self) {
        // Determine the number of child widgets.
        let number_of_child_widgets = self.child_widgets.len();

        // There are no child widgets.
        if number_of_child_widgets <= 0 {
            return;
        }

        // Determine the number of spacers between the child widgets.
        let number_of_spacers = max(number_of_child_widgets - 1, 0);

        // Determine the child size constraints.
        let child_size_constraints =
            SizeConstraints::new(Size::ZERO, *self.size_constraints.maximum());

        let mut child_and_spacing_size_sum = Size::ZERO;
        let mut flex_factor_sum: u16 = 0;

        // First pass over the children.
        for (i, child_widget) in &mut self.child_widgets.iter().enumerate() {
            // Apply the size constraints to the child widget.
            let child_size = RefCell::borrow_mut(&child_widget)
                .borrow_mut()
                .apply_size_constraints(child_size_constraints);

            // Update the sum of child and spacing sizes.
            // Include the child widget's height.
            child_and_spacing_size_sum.height =
                child_and_spacing_size_sum.height.max(child_size.height);

            // Add the spacer to child and spacing sizes.
            if i > 0 {
                child_and_spacing_size_sum.width += self.spacing;
            }

            // Get the child widget's flex factor.
            let flex_factor = RefCell::borrow(&child_widget).borrow().flex_factor();

            // The child widget does not have a flex factor.
            if flex_factor == 0 {
                // Add the child widget's width.
                child_and_spacing_size_sum.width += child_size.width;
            }
            // The child widget does have a flex factor.
            else {
                // Do not add the child widget's width. It will grab the remaining width together
                // with all other widgets having a flex factor.

                // Add the child widget's flex factor.
                flex_factor_sum += flex_factor;
            }
        }

        // The child widgets do not have a flex factor.
        if flex_factor_sum == 0 {
            // Set the parent size to the sum of the child and spacing sizes.
            self.rectangle = self.rectangle.with_size(child_and_spacing_size_sum);
        }
        // The child widgets do have a flex factor.
        else {
            // Set the parent size to the child widget's height and the maximum width.
            self.rectangle = self.rectangle.with_size(Size::new(
                self.size_constraints.maximum().width,
                child_and_spacing_size_sum.height,
            ));
        }

        // Calculate the remaining width.
        let remaining_width = (self.rectangle.width() - child_and_spacing_size_sum.width).max(0.0);

        let mut child_x = self.rectangle.origin().x;

        // Second pass over the children.
        for child_widget in &mut self.child_widgets {
            // Get the child's flex factor.
            let flex_factor = RefCell::borrow(&child_widget).borrow().flex_factor();

            // The child widget does not have a flex factor.
            let child_size = if flex_factor == 0 {
                RefCell::borrow(&child_widget).borrow().size()
            }
            // The child widget does have a flex factor.
            else {
                let child_size = RefCell::borrow(&child_widget).borrow().size();

                // Devide the remaining width among the child widgets with flex factor.
                let expanded_child_size = Size::new(
                    remaining_width * (flex_factor as f64 / flex_factor_sum as f64),
                    child_size.height,
                );

                // Apply the size constraints to the child widget.
                RefCell::borrow_mut(&child_widget)
                    .borrow_mut()
                    .apply_size_constraints(SizeConstraints::tight(expanded_child_size));

                expanded_child_size
            };

            // Determine the child's vertical position.
            let child_y = match self.vertical_alignment {
                VerticalAlignment::Bottom => {
                    self.rectangle.origin().y
                        + (self.rectangle.size().height - child_size.height).max(0.0)
                }
                VerticalAlignment::Middle => {
                    self.rectangle.origin().y
                        + 0.5 * (self.rectangle.size().height - child_size.height).max(0.0)
                }
                VerticalAlignment::Top => self.rectangle.origin().y,
            };

            // Set the children's origins.
            RefCell::borrow_mut(&child_widget)
                .borrow_mut()
                .set_origin((child_x, child_y).into());

            child_x += child_size.width + self.spacing;
        }
    }
}

impl Widget for Row {
    ///
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.size_constraints = size_constraints;

        // Layout the children.
        self.layout_children();

        self.rectangle.size()
    }

    fn handle_command(&mut self, widget_command: WidgetCommand) -> Result<(), WidgetError> {
        match widget_command {
            WidgetCommand::AppendChild(child_widget) => {
                self.child_widgets.push(child_widget);

                // Layout the children.
                self.layout_children();
            }
            WidgetCommand::RemoveAllChildren => {
                self.child_widgets.clear();

                // Layout the children.
                self.layout_children();
            }
            WidgetCommand::RemoveChild(_) => {
                // TODO
                println!("`Row::handle_command(RemoveChild)`: TODO");
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
                println!("`Row::handle_command(SetIsDisabled)`: TODO");
            }
            WidgetCommand::SetIsHidden(is_hidden) => {
                // Hide/show this widget.
                self.is_hidden = is_hidden;
            }
            WidgetCommand::SetStroke(ref _value) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetValue(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetVerticalAlignment(vertical_alignment) => {
                self.vertical_alignment = vertical_alignment;

                // Layout the children.
                self.layout_children();
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: &Event, widget_events: &mut Vec<WidgetEvent>) {
        // Iterate over the child widgets.
        for child_widget in &mut self.child_widgets {
            RefCell::borrow_mut(&child_widget).handle_event(event, widget_events);
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), piet::Error> {
        if self.is_hidden {
            return Ok(());
        }

        // Iterate over the child widgets.
        for child_widget in &self.child_widgets {
            RefCell::borrow(&child_widget).paint(piet, region)?;
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

        // Layout the children.
        self.layout_children();
    }

    fn size(&self) -> Size {
        self.rectangle.size()
    }

    fn widget_id(&self) -> &WidgetId {
        &self.widget_id
    }
}
