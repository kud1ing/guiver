use crate::shared_state::PietSharedState;
use crate::stroke::Stroke;
use crate::widget::widget_core::WidgetCore;
use crate::widget_manager::WidgetBox;
use crate::{Event, PietWidget};
use druid_shell::piet::Piet;
use druid_shell::{kurbo, piet, Region};
use guiver::{
    HorizontalAlignment, Point, Rectangle, Size, SizeConstraints, Widget, WidgetError, WidgetEvent,
    WidgetEventType, WidgetId, WidgetIdProvider, WidgetPlacement,
};
use piet::RenderContext;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;

/// A layout widget that positions its child widgets in a vertical column.
pub struct Column<APP_EVENT: Clone> {
    child_widgets: Vec<WidgetBox<APP_EVENT>>,
    core: WidgetCore<APP_EVENT>,
    horizontal_alignment: HorizontalAlignment,
    spacing: f64,
}

impl<APP_EVENT: Clone> Column<APP_EVENT> {
    ///
    pub fn new(
        widget_id: WidgetId,
        debug_rendering_stroke: Stroke,
        horizontal_alignment: HorizontalAlignment,
        spacing: f64,
    ) -> Self {
        Column {
            child_widgets: vec![],
            core: WidgetCore::new(widget_id, debug_rendering_stroke),
            horizontal_alignment,
            spacing,
        }
    }

    ///
    fn layout_child_widgets(&mut self) {
        // Create the child widget size constraints.
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

            // Update the sum of child widget and spacing sizes.
            // Include the child widget's width.
            child_and_spacing_size_sum.width =
                child_and_spacing_size_sum.width.max(child_size.width);

            // Add the spacer to child widget and spacing sizes.
            if i > 0 {
                child_and_spacing_size_sum.height += self.spacing;
            }

            // Get the child widget's flex factor.
            let flex_factor = RefCell::borrow(child_widget).borrow().flex_factor();

            // The child widget does not have a flex factor.
            if flex_factor == 0 {
                // Add the child widget's height.
                child_and_spacing_size_sum.height += child_size.height;
            }
            // The child widget does have a flex factor.
            else {
                // Do not add the child widget's height. It will grab the remaining height together
                // with all other widgets having a flex factor.

                // Add the child widget's flex factor.
                flex_factor_sum += flex_factor;
            }
        }

        // The child widgets do not have a flex factor.
        if flex_factor_sum == 0 {
            // Set the parent size to the sum of the child widget and spacing sizes.
            self.core.rectangle = self.core.rectangle.with_size(child_and_spacing_size_sum);
        }
        // The child widgets do have a flex factor.
        else {
            // Set the parent size to the child widget's width and the maximum height.
            self.core.rectangle = self.core.rectangle.with_size(child_and_spacing_size_sum);
        }

        // Calculate the remaining width.
        let remaining_height =
            (self.core.rectangle.height() - child_and_spacing_size_sum.height).max(0.0);

        let mut child_y = self.core.rectangle.origin().y;

        // Second pass over the child widgets.
        for child_widget in &mut self.child_widgets {
            // Get the child widget's flex factor.
            let flex_factor = RefCell::borrow(child_widget).borrow().flex_factor();

            // The child widget does not have a flex factor.
            let child_size = if flex_factor == 0 {
                let size = RefCell::borrow(child_widget).borrow().rectangle().size();

                Size::new(size.width, size.height)
            }
            // The child widget does have a flex factor.
            else {
                let child_size = RefCell::borrow(child_widget).borrow().rectangle().size();

                // Devide the remaining height among the child widgets with flex factor.
                let expanded_child_size = Size::new(
                    child_size.width,
                    remaining_height * (flex_factor as f64 / flex_factor_sum as f64),
                );

                // Apply the size constraints to the current child widget.
                RefCell::borrow_mut(child_widget)
                    .borrow_mut()
                    .apply_size_constraints(SizeConstraints::tight(expanded_child_size));

                expanded_child_size
            };

            // Determine the child widget's horizontal position.
            let child_x = match self.horizontal_alignment {
                HorizontalAlignment::Center => {
                    self.core.rectangle.origin().x
                        + 0.5 * (self.core.rectangle.size().width - child_size.width).max(0.0)
                }
                HorizontalAlignment::Left => self.core.rectangle.origin().x,
                HorizontalAlignment::Right => {
                    self.core.rectangle.origin().x + self.core.rectangle.size().width
                        - child_size.width
                }
            };

            // Set the child widget's origins.
            RefCell::borrow_mut(child_widget)
                .borrow_mut()
                .set_origin(Point::new(child_x, child_y));

            child_y += child_size.height + self.spacing;
        }
    }
}

impl<APP_EVENT: Clone> Widget<APP_EVENT> for Column<APP_EVENT> {
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

    fn rectangle(&self) -> &Rectangle {
        &self.core.rectangle
    }

    fn remove_child(&mut self, child_widget_id: WidgetId) -> Result<(), WidgetError> {
        // TODO: Error handling, if the widget does not exist.

        // Remove the widget with the given ID.
        self.child_widgets
            .retain(|child_widget| *RefCell::borrow(child_widget).widget_id() != child_widget_id);

        // Layout the remaining child widgets.
        self.layout_child_widgets();

        Ok(())
    }

    fn remove_children(&mut self) -> Result<(), WidgetError> {
        self.child_widgets.clear();

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

    fn set_horizontal_alignment(
        &mut self,
        horizontal_alignment: HorizontalAlignment,
    ) -> Result<(), WidgetError> {
        self.horizontal_alignment = horizontal_alignment;

        // Layout the child widgets.
        self.layout_child_widgets();

        Ok(())
    }

    fn set_is_disabled(&mut self, _is_disabled: bool) {
        // TODO
        println!("`Column::set_is_disabled()`: TODO");
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

impl<APP_EVENT: Clone> PietWidget<APP_EVENT> for Column<APP_EVENT> {
    fn add_child(
        &mut self,
        widget_placement: Option<WidgetPlacement>,
        child_widget: WidgetBox<APP_EVENT>,
    ) -> Result<(), WidgetError> {
        // A widget placement is given.
        if let Some(widget_placement) = widget_placement {
            match widget_placement {
                WidgetPlacement::After(_widgets_location) => {
                    // TODO
                    println!("TODO: `Column::add_child(WidgetPlacement::After(...))");
                }
                WidgetPlacement::Before(_widgets_location) => {
                    // TODO
                    println!("TODO: `Column::add_child(WidgetPlacement::Before(...))");
                }
                _ => {
                    return Err(WidgetError::NotHandled {
                        widget_id: *self.widget_id(),
                        description: format!("{:?}", widget_placement),
                    })
                }
            }
        }
        // No widget placement is given.
        else {
            self.child_widgets.push(child_widget.clone());
        }

        // Layout the child widgets.
        self.layout_child_widgets();

        Ok(())
    }

    fn handle_event(
        &mut self,
        event: &Event,
        shared_state: &mut PietSharedState,
        widget_id_provider: &mut WidgetIdProvider,
        widget_events: &mut Vec<WidgetEvent<APP_EVENT>>,
    ) {
        // Iterate over the child widgets.
        for child_widget in &mut self.child_widgets {
            // Let the current child widget handle the given event.
            RefCell::borrow_mut(child_widget).handle_event(
                event,
                shared_state,
                widget_id_provider,
                widget_events,
            );
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
