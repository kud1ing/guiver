use crate::PietWidgetBox;
use guiver::{
    Command, Font, HorizontalAlignment, PaintBrush, Stroke, VerticalAlignment, WidgetEventType,
    WidgetId, WidgetPlacement, WidgetType,
};
use std::any::Any;

/// This is only temporary workaround in order to add `AddWidget` until `Command` is generic enough.
pub enum PietCommand<EVENT> {
    AddChild {
        parent_widget_id: WidgetId,
        widget_placement: Option<WidgetPlacement>,
        child_widget_id: WidgetId,
    },
    AddEventObservation(WidgetId, WidgetEventType, EVENT),
    AddChildren {
        parent_widget_id: WidgetId,
        child_widgets: Vec<(Option<WidgetPlacement>, WidgetId)>,
    },
    AddWidget(PietWidgetBox<EVENT>),
    CreateWidget(WidgetId, WidgetType),
    Destroy(WidgetId),
    RemoveChild {
        parent_widget_id: WidgetId,
        child_widget_id: WidgetId,
        destroy_child_widget: bool,
    },
    RemoveChildren {
        parent_widget_id: WidgetId,
        destroy_child_widgets: bool,
    },
    RemoveEventObservation(WidgetId, WidgetEventType),
    SetDebugRendering(WidgetId, bool),
    SetFill(WidgetId, Option<PaintBrush>),
    SetFont(WidgetId, Font),
    SetHasFocus(WidgetId, bool),
    SetHorizontalAlignment(WidgetId, HorizontalAlignment),
    SetIsDisabled(WidgetId, bool),
    SetIsHidden(WidgetId, bool),
    SetMainWidget(WidgetId),
    SetStroke(WidgetId, Option<Stroke>),
    SetValue(WidgetId, Box<dyn Any>),
    SetVerticalAlignment(WidgetId, VerticalAlignment),
}
