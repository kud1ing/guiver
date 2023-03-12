use crate::stroke::Stroke;
use guiver::{Rectangle, SizeConstraints, WidgetEvent, WidgetEventType, WidgetId};
use std::collections::HashMap;

/// Common data and functionality for widgets.
#[derive(Default)]
pub struct WidgetCore<APP_EVENT: Clone> {
    pub debug_rendering: bool,
    pub debug_rendering_stroke: Stroke,
    pub is_hidden: bool,
    pub rectangle: Rectangle,
    pub size_constraints: SizeConstraints,
    widget_event_observation: HashMap<WidgetEventType, WidgetEvent<APP_EVENT>>,
    pub widget_id: WidgetId,
}

impl<APP_EVENT: Clone> WidgetCore<APP_EVENT> {
    ///
    pub fn new(widget_id: WidgetId, debug_rendering_stroke: Stroke) -> Self {
        WidgetCore {
            debug_rendering: false,
            debug_rendering_stroke,
            is_hidden: false,
            rectangle: Rectangle::default(),
            size_constraints: SizeConstraints::unbounded(),
            widget_event_observation: HashMap::new(),
            widget_id,
        }
    }

    ///
    pub fn add_event_observation(
        &mut self,
        widget_event_type: WidgetEventType,
        widget_event: WidgetEvent<APP_EVENT>,
    ) {
        self.widget_event_observation
            .insert(widget_event_type, widget_event);
    }

    ///
    pub fn event_observation(
        &mut self,
        widget_event_type: &WidgetEventType,
    ) -> Option<&WidgetEvent<APP_EVENT>> {
        self.widget_event_observation.get(widget_event_type)
    }

    ///
    pub fn remove_event_observation(&mut self, widget_event_type: &WidgetEventType) {
        self.widget_event_observation.remove(widget_event_type);
    }
}
