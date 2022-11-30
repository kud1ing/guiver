use crate::stroke::Stroke;
use crate::widget::core::WidgetCore;
use crate::widget::{WidgetCommand, WidgetError, WidgetId, WidgetPlacement};
use crate::widget_manager::WidgetBox;
use crate::{Event, SizeConstraints, VerticalAlignment, Widget, WidgetEvent};
use druid_shell::kurbo::{Point, Rect, Size};
use druid_shell::piet::{Piet, RenderContext};
use druid_shell::{piet, Region};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;

/// A layout widget that positions its child widgets in a horizontal row.
pub struct Row {
    child_widgets: Vec<WidgetBox>,
    core: WidgetCore,
    spacing: f64,
    vertical_alignment: VerticalAlignment,
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
            core: WidgetCore::new(widget_id, debug_rendering_stroke),
            spacing,
            vertical_alignment,
        }
    }

    ///
    fn layout_child_widgets(&mut self) {
        // Create the child size constraints.
        let child_size_constraints =
            SizeConstraints::new(Size::ZERO, *self.core.size_constraints.maximum());

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
    }
}

impl Widget for Row {
    ///
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.core.size_constraints = size_constraints;

        // Layout the child widgets.
        self.layout_child_widgets();

        self.core.rectangle.size()
    }

    fn handle_command(&mut self, widget_command: &WidgetCommand) -> Result<(), WidgetError> {
        match widget_command {
            WidgetCommand::AddChild(widget_placement, child_widget) => {
                // A widget placement is given.
                if let Some(widget_placement) = widget_placement {
                    match widget_placement {
                        WidgetPlacement::LeftOf(_widgets_location) => {
                            // TODO
                            println!("TODO: `Column::handle_command(WidgetCommand::AddChild(WidgetPlacement::LeftOf(...)))");
                        }
                        WidgetPlacement::RightOf(_widgets_location) => {
                            // TODO
                            println!("TODO: `Column::handle_command(WidgetCommand::AddChild(WidgetPlacement::RightOf(...)))");
                        }
                        _ => {
                            return self.core.handle_command(widget_command);
                        }
                    }
                }
                // No widget placement is given.
                else {
                    self.child_widgets.push(child_widget.clone());
                }

                // Layout the child widgets.
                self.layout_child_widgets();

                return Ok(());
            }
            WidgetCommand::RemoveAllChildren => {
                self.child_widgets.clear();

                // Update this widget's size.
                self.layout_child_widgets();

                return Ok(());
            }
            WidgetCommand::RemoveChild(child_widget_id) => {
                // TODO: Error handling, if the widget does not exist.

                // Remove the widget with the given ID.
                self.child_widgets.retain(|child_widget| {
                    *RefCell::borrow(child_widget).widget_id() != *child_widget_id
                });

                // Layout the remaining child widgets.
                self.layout_child_widgets();

                return Ok(());
            }
            WidgetCommand::SetVerticalAlignment(vertical_alignment) => {
                self.vertical_alignment = *vertical_alignment;

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
        for child_widget in &mut self.child_widgets {
            // Let the current child widget handle the given event.
            RefCell::borrow_mut(child_widget).handle_event(event, widget_events);
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), piet::Error> {
        // The row widget is hidden.
        if self.core.is_hidden {
            return Ok(());
        }

        // Iterate over the child widgets.
        for child_widget in &self.child_widgets {
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

    fn set_debug_rendering(&mut self, debug_rendering: bool) {
        self.core.debug_rendering = debug_rendering;
    }

    fn set_is_disabled(&mut self, _is_disabled: bool) {
        // TODO
        println!("`Row::set_is_disabled()`: TODO");
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
