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
use std::borrow::{Borrow, BorrowMut};
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
        // Create the child size constraints.
        let child_size_constraints =
            SizeConstraints::new(Size::ZERO, *self.core.size_constraints.maximum());

        // Apply the child size constraints to all child widgets.
        for child_widget in &mut self.child_widgets.values() {
            RefCell::borrow_mut(child_widget)
                .borrow_mut()
                .apply_size_constraints(child_size_constraints);
        }

        let mut column_widths = vec![0.0; self.number_of_columns];
        let mut row_heights = vec![0.0; self.number_of_rows];

        // Determine the column widths and row heights from the widgets.
        {
            // Iterate over the child widgets in order to determine the column widths and row heights.
            for ((column_index, row_index), child_widget) in &self.child_widgets {
                // Get the current child widget's size.
                let widget_size = RefCell::borrow(child_widget).borrow().rectangle().size();

                let current_column_width = column_widths.get(*column_index).unwrap();
                let current_row_height = row_heights.get(*row_index).unwrap();

                // The current widget is wider than the current column width.
                if widget_size.width > *current_column_width {
                    column_widths[*column_index] = widget_size.width;
                }

                // The current widget is higher than the current row height.
                if widget_size.height > *current_row_height {
                    row_heights[*row_index] = widget_size.height;
                }
            }
        }

        let mut child_and_spacing_size_sum = Size::ZERO;
        let mut flex_factor_sum_columns: u16 = 0;
        let mut flex_factor_sum_row: u16 = 0;

        // Determine the column/row flex factors and minimum widths/heights, spacing included.
        {
            // Iterate over the columns.
            for column_index in 0..self.number_of_columns {
                // Add the column width.
                child_and_spacing_size_sum.width = child_and_spacing_size_sum
                    .width
                    .max(*column_widths.get(column_index).unwrap());

                // Get the column properties.
                let column_properties = self.grid_column_properties(column_index);

                if column_index > 0 {
                    // Add the column spacing.
                    child_and_spacing_size_sum.width += column_properties.spacing;
                }

                // Get the column's flex factor.
                let flex_factor = column_properties.flex_factor;

                // The column does not have a flex factor.
                if flex_factor == 0 {
                    // Add the column's width.
                    child_and_spacing_size_sum.width += column_widths.get(column_index).unwrap();
                }
                // The column does have a flex factor.
                else {
                    // Do not add the column's width. It will grab the remaining width together with all
                    // other widgets having a flex factor.

                    // Add the column's flex factor.
                    flex_factor_sum_columns += flex_factor;
                }
            }

            // Iterate over the rows.
            for row_index in 0..self.number_of_rows {
                // Add the row height.
                child_and_spacing_size_sum.height = child_and_spacing_size_sum
                    .height
                    .max(*row_heights.get(row_index).unwrap());

                // Get the row properties.
                let row_properties = self.grid_row_properties(row_index);

                if row_index > 0 {
                    // Add the row spacing.
                    child_and_spacing_size_sum.height += row_properties.spacing;
                }

                // Get the row's flex factor.
                let flex_factor = row_properties.flex_factor;

                // The row does not have a flex factor.
                if flex_factor == 0 {
                    // Add the row's height.
                    child_and_spacing_size_sum.width += row_heights.get(row_index).unwrap();
                }
                // The row does have a flex factor.
                else {
                    // Do not add the row's width. It will grab the remaining width together with all
                    // other widgets having a flex factor.

                    // Add the row's flex factor.
                    flex_factor_sum_row += flex_factor;
                }
            }
        }

        // Set the new parent widget size.
        {
            let mut new_parent_widget_size = child_and_spacing_size_sum;

            // The columns do have flex factors.
            if flex_factor_sum_columns > 0 {
                // TODO: handle the flex factor
            }

            // The rows do have flex factors.
            if flex_factor_sum_row > 0 {
                // TODO: handle the flex factor
            }

            self.core.rectangle = self.core.rectangle.with_size(new_parent_widget_size);
        }

        // Calculate the remaining width and height.
        let remaining_width =
            (self.core.rectangle.width() - child_and_spacing_size_sum.width).max(0.0);
        let remaining_height =
            (self.core.rectangle.height() - child_and_spacing_size_sum.height).max(0.0);

        // TODO
        /*
        let mut child_x = self.core.rectangle.origin().x;
        let mut child_y = self.core.rectangle.origin().y;

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
                column_index: column,
                row_index: row,
                child_widget,
            } => {
                // Add the given child widget.
                self.child_widgets
                    .insert((*column, *row), child_widget.clone());

                // Update the number of columns.
                {
                    let minimum_number_of_column = *column + 1;

                    if minimum_number_of_column > self.number_of_columns {
                        self.number_of_columns = minimum_number_of_column;
                    }
                }

                // Update the number of rows.
                {
                    let minimum_number_of_rows = *row + 1;

                    if minimum_number_of_rows > self.number_of_rows {
                        self.number_of_rows = minimum_number_of_rows;
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
