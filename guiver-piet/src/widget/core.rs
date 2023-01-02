use druid_shell::kurbo::Rect;
use guiver::stroke::Stroke;
use guiver::{SizeConstraints, WidgetEvent, WidgetEventType, WidgetId};
use std::collections::HashMap;

/// Common data and functionality for widgets.
#[derive(Default)]
pub struct WidgetCore<T: Clone> {
    pub debug_rendering: bool,
    pub debug_rendering_stroke: Stroke,
    pub is_hidden: bool,
    pub rectangle: Rect,
    pub size_constraints: SizeConstraints,
    widget_event_observation: HashMap<WidgetEventType, WidgetEvent<T>>,
    pub widget_id: WidgetId,
}

impl<T: Clone> WidgetCore<T> {
    ///
    pub fn new(widget_id: WidgetId, debug_rendering_stroke: Stroke) -> Self {
        WidgetCore {
            debug_rendering: false,
            debug_rendering_stroke,
            is_hidden: false,
            rectangle: Rect::default(),
            size_constraints: SizeConstraints::unbounded(),
            widget_event_observation: HashMap::new(),
            widget_id,
        }
    }

    ///
    pub fn add_event_observation(
        &mut self,
        widget_event_type: WidgetEventType,
        widget_event: WidgetEvent<T>,
    ) {
        self.widget_event_observation
            .insert(widget_event_type, widget_event);
    }

    ///
    pub fn event_observation(
        &mut self,
        widget_event_type: &WidgetEventType,
    ) -> Option<&WidgetEvent<T>> {
        self.widget_event_observation.get(widget_event_type)
    }

    ///
    pub fn remove_event_observation(&mut self, widget_event_type: &WidgetEventType) {
        self.widget_event_observation.remove(widget_event_type);
    }
}
