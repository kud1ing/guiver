use crate::widget::core::WidgetCore;
use crate::widget::WidgetError;
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
use std::collections::{HashMap, HashSet};

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
    child_widget_id_per_cell: HashMap<(usize, usize), WidgetId>,
    child_widget_ids_per_column: HashMap<usize, HashSet<WidgetId>>,
    child_widget_ids_per_row: HashMap<usize, HashSet<WidgetId>>,
    child_widget_per_id: HashMap<WidgetId, WidgetBox>,
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
            child_widget_id_per_cell: HashMap::new(),
            child_widget_ids_per_column: HashMap::new(),
            child_widget_ids_per_row: HashMap::new(),
            child_widget_per_id: HashMap::new(),
            column_properties: vec![],
            core: WidgetCore::new(widget_id, debug_rendering_stroke),
            default_column_properties,
            default_row_properties,
            number_of_columns: 0,
            number_of_rows: 0,
            row_properties: vec![],
        }
    }

    /// Adds the given child widget to the grid.
    fn add_child_widget(&mut self, child_widget: WidgetBox, column_index: usize, row_index: usize) {
        // Get the child widget ID.
        let child_widget_id = *RefCell::borrow(&child_widget).widget_id();

        // Add the child widget.
        self.child_widget_per_id
            .insert(child_widget_id, child_widget);

        // Add the child widget ID.
        {
            self.child_widget_id_per_cell
                .insert((column_index, row_index), child_widget_id);
            self.child_widget_ids_per_column
                .entry(column_index)
                .or_default()
                .insert(child_widget_id);
            self.child_widget_ids_per_row
                .entry(row_index)
                .or_default()
                .insert(child_widget_id);
        }

        // Update the number of columns and rows.
        {
            let minimum_number_of_column = column_index + 1;
            let minimum_number_of_rows = row_index + 1;

            if minimum_number_of_column > self.number_of_columns {
                self.number_of_columns = minimum_number_of_column;
            }
            if minimum_number_of_rows > self.number_of_rows {
                self.number_of_rows = minimum_number_of_rows;
            }
        }

        // Layout the child widgets.
        self.layout_child_widgets();
    }

    ///
    fn layout_child_widgets(&mut self) {
        // Apply size constraints to all child widgets.
        {
            // Create the child size constraints applicable to all child widgets.
            let child_size_constraints =
                SizeConstraints::new(Size::ZERO, *self.core.size_constraints.maximum());

            // Apply the child size constraints to all child widgets.
            for child_widget in &mut self.child_widget_per_id.values() {
                RefCell::borrow_mut(child_widget)
                    .borrow_mut()
                    .apply_size_constraints(child_size_constraints);
            }
        }

        let mut column_widths = vec![0.0; self.number_of_columns];
        let mut row_heights = vec![0.0; self.number_of_rows];

        // Determine the grid's column widths and row heights.
        {
            // Iterate over the child widgets in all cells.
            for ((column_index, row_index), child_widget_id) in &self.child_widget_id_per_cell {
                // Get the child widget in the current cell.
                let child_widget = self.child_widget_per_id.get(child_widget_id).unwrap();

                // Get the child widget's size.
                let widget_size = RefCell::borrow(child_widget).rectangle().size();

                // Update the column width and row height due to the current grid cell.
                {
                    let current_column_width = *column_widths.get(*column_index).unwrap();
                    let current_row_height = *row_heights.get(*row_index).unwrap();

                    // The current widget is wider than the current column width.
                    if widget_size.width > current_column_width {
                        column_widths[*column_index] = widget_size.width;
                    }

                    // The current widget is higher than the current row height.
                    if widget_size.height > current_row_height {
                        row_heights[*row_index] = widget_size.height;
                    }
                }
            }
        }

        let mut child_and_spacing_size_sum = Size::ZERO;
        let mut flex_factor_sum_columns: u16 = 0;
        let mut flex_factor_sum_row: u16 = 0;

        // Determine the column/row flex factors and minimum widths/heights (including spacing).
        {
            // Iterate over the grid's column indices.
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
                    // Do not add the column's width. It will grab the remaining width together with
                    // all other widgets having a flex factor.

                    // Add the column's flex factor.
                    flex_factor_sum_columns += flex_factor;
                }
            }

            // Iterate over the grid's row indices.
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
                    // Do not add the row's width. It will grab the remaining width together with
                    // all other widgets having a flex factor.

                    // Add the row's flex factor.
                    flex_factor_sum_row += flex_factor;
                }
            }
        }

        // Adjust the grid's widget size.
        {
            let new_parent_widget_size = child_and_spacing_size_sum;

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
        let _remaining_width =
            (self.core.rectangle.width() - child_and_spacing_size_sum.width).max(0.0);
        let _remaining_height =
            (self.core.rectangle.height() - child_and_spacing_size_sum.height).max(0.0);

        {
            // Iterate over the grid columns.
            {
                let mut _child_x = self.core.rectangle.origin().x;

                // TODO
            }

            // Iterate over the grid rows.
            {
                let mut _child_y = self.core.rectangle.origin().y;

                // TODO
            }
        }

        /*
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

    /// Removes the child widget with the given ID from the grid.
    fn remove_child_widget(&mut self, child_widget_id: &WidgetId) {
        // Remove the child widget.
        self.child_widget_per_id.remove(child_widget_id);

        // Remove the child widget ID.
        {
            // TODO: Remove from `child_widget_ids_per_cell`

            for (_, child_widgets_in_column) in self.child_widget_ids_per_column.iter_mut() {
                child_widgets_in_column.remove(child_widget_id);
            }

            for (_, child_widgets_in_row) in self.child_widget_ids_per_row.iter_mut() {
                child_widgets_in_row.remove(child_widget_id);
            }
        }

        // Update the number of columns and rows.
        {
            self.number_of_columns = 0;
            self.number_of_rows = 0;

            for (column_index, row_index) in self.child_widget_id_per_cell.keys() {
                let minimum_number_of_column = column_index + 1;
                let minimum_number_of_rows = row_index + 1;

                if minimum_number_of_column > self.number_of_columns {
                    self.number_of_columns = minimum_number_of_column;
                }
                if minimum_number_of_rows > self.number_of_rows {
                    self.number_of_rows = minimum_number_of_rows;
                }
            }
        }

        // Layout the child widgets.
        self.layout_child_widgets();
    }
}

impl Widget for Grid {
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.core.size_constraints = size_constraints;

        // Layout the child widgets.
        self.layout_child_widgets();

        self.core.rectangle.size()
    }

    fn handle_event(&mut self, event: &Event, widget_events: &mut Vec<WidgetEvent>) {
        // Iterate over the child widgets.
        for child_widget in &mut self.child_widget_per_id.values() {
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
        for child_widget in self.child_widget_per_id.values() {
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

    fn remove_all_children(&mut self) -> Result<(), WidgetError> {
        self.child_widget_id_per_cell.clear();
        self.child_widget_ids_per_column.clear();
        self.child_widget_ids_per_row.clear();
        self.child_widget_per_id.clear();

        self.number_of_columns = 0;
        self.number_of_rows = 0;

        // Update this widget's size.
        self.layout_child_widgets();

        Ok(())
    }

    fn set_child(
        &mut self,
        column_index: usize,
        row_index: usize,
        child_widget: WidgetBox,
    ) -> Result<(), WidgetError> {
        self.add_child_widget(child_widget.clone(), column_index, row_index);

        Ok(())
    }

    fn set_debug_rendering(&mut self, debug_rendering: bool) {
        self.core.debug_rendering = debug_rendering;
    }

    fn set_is_disabled(&mut self, _is_disabled: bool) {
        // TODO
        println!("`Grid::set_is_disabled()`: TODO");
    }

    fn set_is_hidden(&mut self, is_hidden: bool) {
        self.core.is_hidden = is_hidden;
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
