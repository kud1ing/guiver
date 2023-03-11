use crate::shared_state::PietSharedState;
use crate::stroke::Stroke;
use crate::widget::widget_core::WidgetCore;
use crate::widget::WidgetError;
use crate::widget_manager::WidgetBox;
use crate::{Event, Piet, PietWidget};
use druid_shell::piet::{Error, RenderContext};
use druid_shell::{kurbo, Region};
use guiver::{
    GridColumnProperties, GridRowProperties, Point, Rect, Size, SizeConstraints, Widget,
    WidgetEvent, WidgetEventType, WidgetId, WidgetIdProvider, WidgetPlacement,
};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

// =================================================================================================

/// A layout widget that positions its child widgets in a 2-dimensional grid.
pub struct Grid<APP_EVENT: Clone> {
    child_widget_id_per_cell: HashMap<(usize, usize), WidgetId>,
    child_widget_ids_per_column: HashMap<usize, HashSet<WidgetId>>,
    child_widget_ids_per_row: HashMap<usize, HashSet<WidgetId>>,
    child_widget_per_id: HashMap<WidgetId, WidgetBox<APP_EVENT>>,
    column_properties: Vec<GridColumnProperties>,
    core: WidgetCore<APP_EVENT>,
    default_column_properties: GridColumnProperties,
    default_row_properties: GridRowProperties,
    number_of_columns: usize,
    number_of_rows: usize,
    row_properties: Vec<GridRowProperties>,
}

impl<APP_EVENT: Clone> Grid<APP_EVENT> {
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
    fn add_child_widget(
        &mut self,
        child_widget: WidgetBox<APP_EVENT>,
        column_index: usize,
        row_index: usize,
    ) {
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
                    child_and_spacing_size_sum.height += row_heights.get(row_index).unwrap();
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

        // Set the child widget positions.
        {
            // Iterate over the grid columns.
            {
                let mut child_x = self.core.rectangle.origin().x;

                // Iterate over the grid's column indices.
                for column_index in 0..self.number_of_columns {
                    // Add the column spacing.
                    if column_index > 0 {
                        child_x += self.grid_column_properties(column_index).spacing;
                    }

                    // Iterate over the child widgets in the current column.
                    for child_widget_id_in_column in self
                        .child_widget_ids_per_column
                        .get(&column_index)
                        .unwrap_or(&HashSet::new())
                    {
                        // Get the current child widget in the current column.
                        let child_widget_in_column = self
                            .child_widget_per_id
                            .get(child_widget_id_in_column)
                            .unwrap();

                        // Set the child widget's x position.
                        RefCell::borrow_mut(child_widget_in_column)
                            .borrow_mut()
                            .set_origin(Point::new(
                                child_x,
                                child_widget_in_column.borrow().rectangle().y0,
                            ));
                    }

                    // Add the column width.
                    child_x += column_widths.get(column_index).unwrap();
                }
            }

            // Iterate over the grid rows.
            {
                let mut child_y = self.core.rectangle.origin().y;

                // Iterate over the grid's row indices.
                for row_index in 0..self.number_of_rows {
                    // Add the row spacing.
                    if row_index > 0 {
                        child_y += self.grid_row_properties(row_index).spacing;
                    }

                    // Iterate the child widgets in the current row.
                    for child_widget_id_in_row in self
                        .child_widget_ids_per_row
                        .get(&row_index)
                        .unwrap_or(&HashSet::new())
                    {
                        // Get the current child widget in the current row.
                        let child_widget_in_row = self
                            .child_widget_per_id
                            .get(child_widget_id_in_row)
                            .unwrap();

                        // Set the child widget's y position.
                        RefCell::borrow_mut(child_widget_in_row)
                            .borrow_mut()
                            .set_origin(Point::new(
                                child_widget_in_row.borrow().rectangle().x0,
                                child_y,
                            ));
                    }

                    // Add the row height.
                    child_y += row_heights.get(row_index).unwrap();
                }
            }
        }
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

impl<APP_EVENT: Clone> Widget<APP_EVENT> for Grid<APP_EVENT> {
    fn add_event_observation(
        &mut self,
        widget_event_type: WidgetEventType,
        widget_event: WidgetEvent<APP_EVENT>,
    ) {
        self.core
            .add_event_observation(widget_event_type, widget_event);
    }

    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.core.size_constraints = size_constraints;

        // Layout the child widgets.
        self.layout_child_widgets();

        let size = self.core.rectangle.size();

        Size::new(size.width, size.height)
    }

    fn event_observation(
        &mut self,
        widget_event_type: &WidgetEventType,
    ) -> Option<&WidgetEvent<APP_EVENT>> {
        self.core.event_observation(widget_event_type)
    }

    fn rectangle(&self) -> &Rect {
        &self.core.rectangle
    }

    fn remove_children(&mut self) -> Result<(), WidgetError> {
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

    fn remove_event_observation(&mut self, widget_event_type: &WidgetEventType) {
        self.core.remove_event_observation(widget_event_type);
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

impl<APP_EVENT: Clone> PietWidget<APP_EVENT> for Grid<APP_EVENT> {
    fn add_child(
        &mut self,
        widget_placement: Option<WidgetPlacement>,
        child_widget: WidgetBox<APP_EVENT>,
    ) -> Result<(), WidgetError> {
        // A grid widget placement is given
        if let Some(WidgetPlacement::Grid {
            column_index,
            row_index,
        }) = widget_placement
        {
            self.add_child_widget(child_widget.clone(), column_index, row_index);

            return Ok(());
        }

        Err(WidgetError::NotHandled {
            widget_id: self.core.widget_id,
            description: format!("`add_child({:?})`", widget_placement),
        })
    }

    fn handle_event(
        &mut self,
        event: &Event,
        shared_state: &mut PietSharedState,
        widget_id_provider: &mut WidgetIdProvider,
        widget_events: &mut Vec<WidgetEvent<APP_EVENT>>,
    ) {
        // Iterate over the child widgets.
        for child_widget in &mut self.child_widget_per_id.values() {
            // Let the current child widget handle the given event.
            RefCell::borrow_mut(child_widget).handle_event(
                event,
                shared_state,
                widget_id_provider,
                widget_events,
            );
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
                kurbo::Rect::new(
                    self.core.rectangle.x0,
                    self.core.rectangle.y0,
                    self.core.rectangle.x1,
                    self.core.rectangle.y1,
                ),
                &self.core.debug_rendering_stroke.stroke_brush,
                self.core.debug_rendering_stroke.stroke_width,
            );
        }

        Ok(())
    }
}
