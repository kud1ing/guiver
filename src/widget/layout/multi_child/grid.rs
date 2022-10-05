use crate::widget::core::WidgetCore;
use crate::widget::{WidgetCommand, WidgetError};
use crate::widget_manager::WidgetBox;
use crate::{
    Event, HorizontalAlignment, Piet, Size, SizeConstraints, Stroke, VerticalAlignment, Widget,
    WidgetEvent, WidgetId,
};
use druid_shell::kurbo::{Point, Rect};
use druid_shell::piet::{Error, RenderContext};
use druid_shell::Region;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;

///
pub struct GridColumnProperties {
    flex_factor: u16,
    horizontal_alignment: HorizontalAlignment,
    minimum_width: f64,
    spacing: f64,
}

impl Default for GridColumnProperties {
    fn default() -> Self {
        GridColumnProperties {
            flex_factor: 0,
            horizontal_alignment: HorizontalAlignment::Center,
            minimum_width: 0.0,
            spacing: 10.0,
        }
    }
}

// =================================================================================================

///
pub struct GridRowProperties {
    flex_factor: u16,
    minimum_height: f64,
    spacing: f64,
    vertical_alignment: VerticalAlignment,
}

impl Default for GridRowProperties {
    fn default() -> Self {
        GridRowProperties {
            flex_factor: 0,
            minimum_height: 0.0,
            spacing: 10.0,
            vertical_alignment: VerticalAlignment::Middle,
        }
    }
}

// =================================================================================================

/// A layout widget that positions its child widgets in a 2-dimensional grid.
pub struct Grid {
    child_widgets: HashMap<(usize, usize), WidgetBox>,
    column_properties: Vec<GridColumnProperties>,
    core: WidgetCore,
    default_column_properties: GridColumnProperties,
    default_row_properties: GridRowProperties,
    number_of_columns: usize,
    number_of_rows: usize,
    row_properties: Vec<GridRowProperties>,
}

impl Grid {
    ///
    pub fn new(
        widget_id: WidgetId,
        debug_rendering_stroke: Stroke,
        default_column_properties: GridColumnProperties,
        default_row_properties: GridRowProperties,
    ) -> Self {
        Grid {
            child_widgets: HashMap::new(),
            column_properties: vec![],
            core: WidgetCore::new(widget_id, debug_rendering_stroke),
            default_column_properties,
            default_row_properties,
            number_of_columns: 0,
            number_of_rows: 0,
            row_properties: vec![],
        }
    }

    ///
    fn layout_child_widgets(&mut self) {
        // There are no child widgets.
        if self.child_widgets.is_empty() {
            return;
        }

        // Create the child size constraints.
        let child_size_constraints =
            SizeConstraints::new(Size::ZERO, *self.core.size_constraints.maximum());

        // Iterate over the child widgets.
        for child_widget in &mut self.child_widgets.values() {
            // Apply the size constraints to the current child widget.
            RefCell::borrow_mut(child_widget)
                .borrow_mut()
                .apply_size_constraints(child_size_constraints);
        }

        for _column_index in 0..self.number_of_columns {
            for _row_index in 0..self.number_of_rows {
                // TODO: Determine the column widths. Use `grid_column_properties()`
                // TODO: Determine the row heights. Use `grid_row_properties()`
            }
        }

        /*
        let mut child_and_spacing_size_sum = Size::ZERO;
        let mut flex_factor_sum: u16 = 0;

        // First pass over the child widgets.
        for (i, child_widget) in &mut self.child_widgets.iter().enumerate() {
            // Apply the size constraints to the current child widget.
            let child_size = RefCell::borrow_mut(child_widget)
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
            let flex_factor = RefCell::borrow(child_widget).borrow().flex_factor();

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
            self.core.rectangle = self.core.rectangle.with_size(child_and_spacing_size_sum);
        }
        // The child widgets do have a flex factor.
        else {
            // Set the parent size to the child widget's height and the maximum width.
            self.core.rectangle = self.core.rectangle.with_size(Size::new(
                self.core.size_constraints.maximum().width,
                child_and_spacing_size_sum.height,
            ));
        }

        // Calculate the remaining width.
        let remaining_width =
            (self.core.rectangle.width() - child_and_spacing_size_sum.width).max(0.0);

        let mut child_x = self.core.rectangle.origin().x;

        // Second pass over the child widgets.
        for child_widget in &mut self.child_widgets {
            // Get the child widget's flex factor.
            let flex_factor = RefCell::borrow(child_widget).borrow().flex_factor();

            // The child widget does not have a flex factor.
            let child_size = if flex_factor == 0 {
                RefCell::borrow(child_widget).borrow().rectangle().size()
            }
            // The child widget does have a flex factor.
            else {
                let child_size = RefCell::borrow(child_widget).borrow().rectangle().size();

                // Devide the remaining width among the child widgets with flex factor.
                let expanded_child_size = Size::new(
                    remaining_width * (flex_factor as f64 / flex_factor_sum as f64),
                    child_size.height,
                );

                // Apply the size constraints to the current child widget.
                RefCell::borrow_mut(child_widget)
                    .borrow_mut()
                    .apply_size_constraints(SizeConstraints::tight(expanded_child_size));

                expanded_child_size
            };

            // Determine the child widget's vertical position.
            let child_y = match self.vertical_alignment {
                VerticalAlignment::Bottom => {
                    self.core.rectangle.origin().y
                        + (self.core.rectangle.size().height - child_size.height).max(0.0)
                }
                VerticalAlignment::Middle => {
                    self.core.rectangle.origin().y
                        + 0.5 * (self.core.rectangle.size().height - child_size.height).max(0.0)
                }
                VerticalAlignment::Top => self.core.rectangle.origin().y,
            };

            // Set the child widget's origins.
            RefCell::borrow_mut(child_widget)
                .borrow_mut()
                .set_origin((child_x, child_y).into());

            child_x += child_size.width + self.spacing;
        }
        */
    }

    ///
    fn grid_column_properties(&self, column_index: usize) -> &GridColumnProperties {
        self.column_properties
            .get(column_index)
            .unwrap_or(&self.default_column_properties)
    }

    ///
    fn grid_row_properties(&self, row_index: usize) -> &GridRowProperties {
        self.row_properties
            .get(row_index)
            .unwrap_or(&self.default_row_properties)
    }
}

impl Widget for Grid {
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.core.size_constraints = size_constraints;

        // Layout the child widgets.
        self.layout_child_widgets();

        self.core.rectangle.size()
    }

    fn handle_command(&mut self, widget_command: &WidgetCommand) -> Result<(), WidgetError> {
        match widget_command {
            WidgetCommand::SetChild {
                column,
                row,
                child_widget,
            } => {
                // Add the given child widget.
                self.child_widgets
                    .insert((*column, *row), child_widget.clone());

                // Update the number of columns and rows.
                {
                    if *column > self.number_of_columns {
                        self.number_of_columns = *column;
                    }
                    if *row > self.number_of_rows {
                        self.number_of_rows = *row;
                    }
                }

                // Layout the child widgets.
                self.layout_child_widgets();

                return Ok(());
            }
            WidgetCommand::RemoveAllChildren => {
                self.child_widgets.clear();
                self.number_of_columns = 0;
                self.number_of_rows = 0;

                // Update this widget's size.
                self.layout_child_widgets();

                return Ok(());
            }
            WidgetCommand::RemoveChild(_) => {
                // TODO: Remove the child widget.
                println!("TODO: `Grid::handle_command(RemoveChild)`");

                // TODO: Update the number of columns and rows.

                // Layout the child widgets.
                self.layout_child_widgets();

                return Ok(());
            }
            _ => {}
        }

        self.core.handle_command(widget_command)
    }

    fn handle_event(&mut self, event: &Event, widget_events: &mut Vec<WidgetEvent>) {
        // Iterate over the child widgets.
        for child_widget in &mut self.child_widgets.values() {
            // Let the current child widget handle the given event.
            RefCell::borrow_mut(child_widget).handle_event(event, widget_events);
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), Error> {
        // The grid widget is hidden.
        if self.core.is_hidden {
            return Ok(());
        }

        // Iterate over the child widgets.
        for child_widget in self.child_widgets.values() {
            // Paint the current child widget.
            RefCell::borrow(child_widget).paint(piet, region)?;
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

        // Layout the child widgets.
        self.layout_child_widgets();
    }

    fn widget_id(&self) -> &WidgetId {
        &self.core.widget_id
    }
}
