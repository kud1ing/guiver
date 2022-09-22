use crate::widget::core::WidgetCore;
use crate::widget::{WidgetCommand, WidgetError, WidgetPlacement};
use crate::{Event, Piet, Size, SizeConstraints, Stroke, Widget, WidgetEvent, WidgetId};
use druid_shell::kurbo::{Point, Rect};
use druid_shell::piet::{Error, RenderContext};
use druid_shell::Region;

///
pub struct GridColumnProperties {
    flex_factor: u16,
    minimum_width: f64,
    //number_of_columns: usize,
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
    column_properties: GridColumnProperties,
    core: WidgetCore,
    row_properties: GridRowProperties,
}

impl Grid {
    ///
    pub fn new(
        widget_id: WidgetId,
        debug_rendering_stroke: Stroke,
        column_properties: GridColumnProperties,
        row_properties: GridRowProperties,
    ) -> Self {
        Grid {
            column_properties,
            core: WidgetCore::new(widget_id, debug_rendering_stroke),
            row_properties,
        }
    }

    ///
    fn layout_children(&mut self) {
        // TODO
    }
}

impl Widget for Grid {
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.core.size_constraints = size_constraints;

        // Layout the children.
        self.layout_children();

        self.core.rectangle.size()
    }

    fn handle_command(&mut self, widget_command: &WidgetCommand) -> Result<(), WidgetError> {
        match widget_command {
            WidgetCommand::SetChild(widget_placement, _) => {
                match widget_placement {
                    WidgetPlacement::AtCell { .. } => {
                        // TODO
                        println!("TODO: `Grid::handle_command(SetChild)`");
                        return Ok(());
                    }
                    _ => {}
                }
            }
            WidgetCommand::RemoveAllChildren => {
                // TODO
                println!("TODO: `Grid::handle_command(RemoveAllChildren)`");
                return Ok(());
            }
            WidgetCommand::RemoveChild(_) => {
                // TODO
                println!("TODO: `Grid::handle_command(RemoveChild)`");
                return Ok(());
            }
            _ => {}
        }

        self.core.handle_command(widget_command)
    }

    fn handle_event(&mut self, event: &Event, widget_events: &mut Vec<WidgetEvent>) {
        // TODO: Iterate over the child widgets.
        println!("TODO: `Grid::handle_event()`");
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), Error> {
        if self.core.is_hidden {
            return Ok(());
        }

        // TODO: Iterate over the child widgets.
        println!("TODO: `Grid::paint()`");

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

        // Layout the children.
        self.layout_children();
    }

    fn widget_id(&self) -> &WidgetId {
        &self.core.widget_id
    }
}
