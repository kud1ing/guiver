use crate::shared_state::PietSharedState;
use crate::stroke::Stroke;
use crate::widget::widget_core::WidgetCore;
use crate::widget_manager::WidgetBox;
use crate::{Event, Piet, PietWidget};
use druid_shell::piet::{Error, RenderContext};
use druid_shell::{kurbo, Region};
use guiver::{
    Point, Rectangle, Size, SizeConstraints, Widget, WidgetError, WidgetEvent, WidgetEventType,
    WidgetId, WidgetIdProvider, WidgetPlacement,
};

/// A layout widget that tries to adjust its child widget to a desired size.
pub struct SizedBox<APP_EVENT: Clone> {
    child_widget: Option<WidgetBox<APP_EVENT>>,
    core: WidgetCore<APP_EVENT>,
    desired_size: Size,
}

impl<APP_EVENT: Clone> SizedBox<APP_EVENT> {
    ///
    pub fn new(widget_id: WidgetId, debug_rendering_stroke: Stroke, desired_size: Size) -> Self {
        SizedBox {
            child_widget: None,
            core: WidgetCore::new(widget_id, debug_rendering_stroke),
            desired_size,
        }
    }

    ///
    fn layout_child_widget(&mut self) {
        let widget_size = self.desired_size.clamp(
            *self.core.size_constraints.minimum(),
            *self.core.size_constraints.maximum(),
        );

        self.core.rectangle = self.core.rectangle.with_size(widget_size);

        // There is a child widget.
        if let Some(child_widget) = &mut self.child_widget {
            // Apply the child widget's size constraints.
            child_widget
                .borrow_mut()
                .apply_size_constraints(SizeConstraints::tight(widget_size));

            // Set the child widget's origin.
            child_widget
                .borrow_mut()
                .set_origin(self.core.rectangle.origin());
        }
    }
}

impl<APP_EVENT: Clone> Widget<APP_EVENT> for SizedBox<APP_EVENT> {
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

        // Layout the child widget.
        self.layout_child_widget();

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

    fn remove_children(&mut self) -> Result<(), WidgetError> {
        self.child_widget = None;

        // Update this widget's size.
        self.layout_child_widget();

        Ok(())
    }

    fn remove_child(&mut self, _child_widget_id: WidgetId) -> Result<(), WidgetError> {
        // TODO
        println!("`SizedBox::remove_child()`: TODO");

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
        println!("`SizedBox::set_is_disabled()`: TODO");
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

impl<APP_EVENT: Clone> PietWidget<APP_EVENT> for SizedBox<APP_EVENT> {
    fn add_child(
        &mut self,
        _widget_placement: Option<WidgetPlacement>,
        child_widget: WidgetBox<APP_EVENT>,
    ) -> Result<(), WidgetError> {
        // TODO: use `_widget_placement`?

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
        widget_events: &mut Vec<WidgetEvent<APP_EVENT>>,
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

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), Error> {
        // The sized box widget is hidden.
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
