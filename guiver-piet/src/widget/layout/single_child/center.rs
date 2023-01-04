use crate::shared_state::PietSharedState;
use crate::widget::core::WidgetCore;
use crate::widget_manager::WidgetBox;
use crate::{Event, PietWidget};
use druid_shell::kurbo::{Point, Rect, Size};
use druid_shell::piet::{Piet, RenderContext};
use druid_shell::{piet, Region};
use guiver::stroke::Stroke;
use guiver::{
    SizeConstraints, Widget, WidgetError, WidgetEvent, WidgetEventType, WidgetId, WidgetIdProvider,
    WidgetPlacement,
};

/// A layout widget that centers its child widget.
pub struct Center<EVENT: Clone> {
    child_widget: Option<WidgetBox<EVENT>>,
    core: WidgetCore<EVENT>,
}

impl<EVENT: Clone> Center<EVENT> {
    ///
    pub fn new(widget_id: WidgetId, debug_rendering_stroke: Stroke) -> Self {
        Center {
            child_widget: None,
            core: WidgetCore::new(widget_id, debug_rendering_stroke),
        }
    }

    ///
    fn layout_child_widget(&mut self) {
        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            // Apply the child widget's size constraints.
            let child_size = child_widget
                .borrow_mut()
                .apply_size_constraints(self.core.size_constraints);

            self.core.rectangle = self.core.rectangle.with_size(child_size.clamp(
                *self.core.size_constraints.minimum(),
                *self.core.size_constraints.maximum(),
            ));

            // Set the child widget's origin.
            child_widget.borrow_mut().set_origin(
                self.core.rectangle.origin()
                    + (
                        0.5 * (self.core.rectangle.size().width - child_size.width).max(0.0),
                        0.5 * (self.core.rectangle.size().height - child_size.height).max(0.0),
                    ),
            );
        } else {
            self.core.rectangle = self
                .core
                .rectangle
                .with_size(*self.core.size_constraints.minimum());
        }
    }
}

impl<EVENT: Clone> Widget<EVENT> for Center<EVENT> {
    fn add_event_observation(
        &mut self,
        widget_event_type: WidgetEventType,
        widget_event: WidgetEvent<EVENT>,
    ) {
        self.core
            .add_event_observation(widget_event_type, widget_event);
    }

    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.core.size_constraints = size_constraints;

        // Layout the child widget.
        self.layout_child_widget();

        self.core.rectangle.size()
    }

    fn event_observation(
        &mut self,
        widget_event_type: &WidgetEventType,
    ) -> Option<&WidgetEvent<EVENT>> {
        self.core.event_observation(widget_event_type)
    }

    fn rectangle(&self) -> &Rect {
        &self.core.rectangle
    }

    fn remove_children(&mut self) -> Result<(), WidgetError> {
        self.child_widget = None;

        // Update this widget's size.
        self.layout_child_widget();

        Ok(())
    }

    fn remove_child(&mut self, _child_widget_id: WidgetId) -> Result<(), WidgetError> {
        // TODO
        println!("`Center::remove_child()`: TODO");

        // Update this widget's size.
        self.layout_child_widget();

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
        println!("`Center::set_is_disabled()`: TODO");
    }

    fn set_is_hidden(&mut self, is_hidden: bool) {
        self.core.is_hidden = is_hidden;
    }

    fn set_origin(&mut self, origin: Point) {
        self.core.rectangle = self.core.rectangle.with_origin(origin);

        // Layout the child widget.
        self.layout_child_widget();
    }

    fn widget_id(&self) -> &WidgetId {
        &self.core.widget_id
    }
}

impl<EVENT: Clone> PietWidget<EVENT> for Center<EVENT> {
    fn add_child(
        &mut self,
        _widget_placement: Option<WidgetPlacement>,
        child_widget: WidgetBox<EVENT>,
    ) -> Result<(), WidgetError> {
        self.child_widget = Some(child_widget);

        // Layout the child widget.
        self.layout_child_widget();

        Ok(())
    }

    fn handle_event(
        &mut self,
        event: &Event,
        shared_state: &mut PietSharedState,
        widget_id_provider: &mut WidgetIdProvider,
        widget_events: &mut Vec<WidgetEvent<EVENT>>,
    ) {
        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            // Let the child widget handle the event.
            child_widget.borrow_mut().handle_event(
                event,
                shared_state,
                widget_id_provider,
                widget_events,
            )
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), piet::Error> {
        // The center widget is hidden.
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
}
