use crate::shared_state::SharedState;
use crate::widget::core::WidgetCore;

use crate::widget_manager::WidgetBox;
use crate::{Event, Piet, Widget};
use druid_shell::kurbo::{Point, Rect};
use druid_shell::piet::{Error, RenderContext};
use druid_shell::Region;
use guiver::stroke::Stroke;
use guiver::{Size, SizeConstraints, WidgetError, WidgetEvent, WidgetId, WidgetPlacement};

/// A layout widget that tries to adjust its child widget to take all of the available space.
/// Mostly useful in `Column` and `Row`.
pub struct Expanded {
    child_widget: Option<WidgetBox>,
    core: WidgetCore,
    flex_factor: u16,
}

impl Expanded {
    ///
    pub fn new(widget_id: WidgetId, debug_rendering_stroke: Stroke, flex_factor: u16) -> Self {
        Expanded {
            child_widget: None,
            core: WidgetCore::new(widget_id, debug_rendering_stroke),
            flex_factor,
        }
    }

    ///
    fn layout_child_widget(&mut self) {
        self.core.rectangle = self
            .core
            .rectangle
            .with_size(*self.core.size_constraints.maximum());

        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            // Set the child widget's size.
            child_widget
                .borrow_mut()
                .apply_size_constraints(SizeConstraints::tight(self.core.rectangle.size()));

            // Set the child widget's origin.
            child_widget
                .borrow_mut()
                .set_origin(self.core.rectangle.origin());
        }
    }
}

impl Widget for Expanded {
    fn add_child(
        &mut self,
        _widget_placement: Option<WidgetPlacement>,
        child_widget: WidgetBox,
    ) -> Result<(), WidgetError> {
        self.child_widget = Some(child_widget.clone());

        // Layout the child.
        self.layout_child_widget();

        return Ok(());
    }

    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.core.size_constraints = size_constraints;

        // Layout the child.
        self.layout_child_widget();

        self.core.rectangle.size()
    }

    fn flex_factor(&self) -> u16 {
        self.flex_factor
    }

    fn handle_event(
        &mut self,
        shared_state: &mut SharedState,
        event: &Event,
        widget_events: &mut Vec<WidgetEvent>,
    ) {
        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            child_widget
                .borrow_mut()
                .handle_event(shared_state, event, widget_events);
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), Error> {
        // The expanded widget is hidden.
        if self.core.is_hidden {
            return Ok(());
        }

        // There is a child widget.
        if let Some(child_widget_rc) = &self.child_widget {
            // Paint the child widget.
            child_widget_rc.borrow().paint(piet, region)?;
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

    fn remove_children(&mut self) -> Result<(), WidgetError> {
        self.child_widget = None;

        // Update this widget's size.
        self.layout_child_widget();

        return Ok(());
    }

    fn remove_child(&mut self, child_widget_id: WidgetId) -> Result<(), WidgetError> {
        // There is a child widget.
        if let Some(_current_child_widget) = &mut self.child_widget {
            // TODO
            println!("`Expanded::remove_child()`: TODO");

            // Update this widget's size.
            self.layout_child_widget();

            return Ok(());
        }
        // There is no child widget.
        else {
            return Err(WidgetError::NoSuchChildWidget {
                parent_widget_id: self.widget_id().clone(),
                child_widget_id,
            });
        }
    }

    fn set_debug_rendering(&mut self, debug_rendering: bool) {
        self.core.debug_rendering = debug_rendering;
    }

    fn set_is_disabled(&mut self, _is_disabled: bool) {
        // TODO
        println!("`Expanded::set_is_disabled()`: TODO");
    }

    fn set_is_hidden(&mut self, is_hidden: bool) {
        self.core.is_hidden = is_hidden;
    }

    fn set_origin(&mut self, origin: Point) {
        self.core.rectangle = self.core.rectangle.with_origin(origin);

        // Layout the child.
        self.layout_child_widget();
    }

    fn widget_id(&self) -> &WidgetId {
        &self.core.widget_id
    }
}
