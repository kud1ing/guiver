use crate::{
    Font, HorizontalAlignment, Stroke, VerticalAlignment, WidgetEventType, WidgetId,
    WidgetPlacement, WidgetType,
};
use druid_shell::piet::PaintBrush;
use std::any::Any;

/// A command to the widget manager.
#[derive(Debug)]
pub enum Command<EVENT> {
    /// Adds the child widget with the given ID to the parent widget.
    AddChild {
        parent_widget_id: WidgetId,
        widget_placement: Option<WidgetPlacement>,
        child_widget_id: WidgetId,
    },
    /// Adds widget event observation: if a widget event of type `WidgetEventType` occurs in the
    /// widget with the given ID, it produces a value of type `WidgetEvent::Custom(EVENT)` in
    /// `handle_event()`.
    AddEventObservation(WidgetId, WidgetEventType, EVENT),
    /// Adds the child widgets to the parent widget.
    AddChildren {
        parent_widget_id: WidgetId,
        child_widgets: Vec<(Option<WidgetPlacement>, WidgetId)>,
    },
    /// Creates a widget with the given ID and type.
    CreateWidget(WidgetId, WidgetType),
    /// Destroys the widget with the given ID.
    Destroy(WidgetId),
    /// Removes the child widget with the given ID from the parent widget.
    RemoveChild {
        parent_widget_id: WidgetId,
        child_widget_id: WidgetId,
        destroy_child_widget: bool,
    },
    /// Removes the widget's child widgets.
    RemoveChildren {
        parent_widget_id: WidgetId,
        destroy_child_widgets: bool,
    },
    /// Removes observation of the given widget's event.
    RemoveEventObservation(WidgetId, WidgetEventType),
    /// Enables/disables debug rendering mode for the widget.
    SetDebugRendering(WidgetId, bool),
    /// Sets/unsets the widget's fill.
    SetFill(WidgetId, Option<PaintBrush>),
    /// Sets the widget's font.
    SetFont(WidgetId, Font),
    /// Gives/removes focus to the widget.
    SetHasFocus(WidgetId, bool),
    /// Sets the widget's horizontal alignment.
    SetHorizontalAlignment(WidgetId, HorizontalAlignment),
    /// Enables/disables the widget.
    SetIsDisabled(WidgetId, bool),
    /// Hides/shows the widget.
    SetIsHidden(WidgetId, bool),
    /// Makes the widget with the given ID the main widget.
    SetMainWidget(WidgetId),
    /// Sets/unsets the widget's stroke.
    SetStroke(WidgetId, Option<Stroke>),
    /// Sets the given value to the widget.
    SetValue(WidgetId, Box<dyn Any>),
    /// Sets the widget's vertical alignment.
    SetVerticalAlignment(WidgetId, VerticalAlignment),
}

impl<EVENT> Command<EVENT> {
    /// Returns the ID of the receiver widget.
    pub fn widget_id(&self) -> &WidgetId {
        match self {
            Command::AddChild {
                parent_widget_id, ..
            } => parent_widget_id,
            Command::AddChildren {
                parent_widget_id, ..
            } => parent_widget_id,
            Command::AddEventObservation(widget_id, _, _) => widget_id,
            Command::CreateWidget(widget_id, _) => widget_id,
            Command::Destroy(widget_id) => widget_id,
            Command::RemoveChild {
                parent_widget_id, ..
            } => parent_widget_id,
            Command::RemoveChildren {
                parent_widget_id, ..
            } => parent_widget_id,
            Command::RemoveEventObservation(widget_id, _) => widget_id,
            Command::SetDebugRendering(widget_id, _) => widget_id,
            Command::SetFill(widget_id, _) => widget_id,
            Command::SetFont(widget_id, _) => widget_id,
            Command::SetHasFocus(widget_id, _) => widget_id,
            Command::SetHorizontalAlignment(widget_id, _) => widget_id,
            Command::SetIsDisabled(widget_id, _) => widget_id,
            Command::SetIsHidden(widget_id, _) => widget_id,
            Command::SetMainWidget(widget_id) => widget_id,
            Command::SetStroke(widget_id, _) => widget_id,
            Command::SetValue(widget_id, _) => widget_id,
            Command::SetVerticalAlignment(widget_id, _) => widget_id,
        }
    }
}
