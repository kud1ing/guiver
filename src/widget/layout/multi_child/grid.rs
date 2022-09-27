use crate::widget::core::WidgetCore;
use crate::widget::{WidgetCommand, WidgetError};
use crate::widget_manager::WidgetBox;
use crate::{Event, Piet, Size, SizeConstraints, Stroke, Widget, WidgetEvent, WidgetId};
use druid_shell::kurbo::{Point, Rect};
use druid_shell::piet::{Error, RenderContext};
use druid_shell::Region;
use std::cell::RefCell;
use std::collections::HashMap;

///
pub struct GridColumnProperties {
    flex_factor: u16,
    minimum_width: f64,
    spacing: f64,
}

impl Default for GridColumnProperties {
    fn default() -> Self {
        GridColumnProperties {
            flex_factor: 1,
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
}

impl Default for GridRowProperties {
    fn default() -> Self {
        GridRowProperties {
            flex_factor: 1,
            minimum_height: 0.0,
            spacing: 10.0,
        }
    }
}

// =================================================================================================

/// A layout widget that positions its child widgets in a 2-dimensional grid.
pub struct Grid {
    child_widgets: HashMap<(usize, usize), WidgetBox>,
    column_properties: Vec<GridColumnProperties>,
    default_column_properties: GridColumnProperties,
    default_row_properties: GridRowProperties,
    core: WidgetCore,
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
        // TODO: Determine the column widths. Use `grid_column_properties()`
        // TODO: Determine the row heights. Use `grid_row_properties()`

        // TODO
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

                self.layout_child_widgets();

                return Ok(());
            }
            WidgetCommand::RemoveAllChildren => {
                self.child_widgets.clear();
                self.number_of_columns = 0;
                self.number_of_rows = 0;

                self.layout_child_widgets();

                return Ok(());
            }
            WidgetCommand::RemoveChild(_) => {
                // TODO
                println!("TODO: `Grid::handle_command(RemoveChild)`");

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
