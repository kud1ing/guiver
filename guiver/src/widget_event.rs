use crate::WidgetId;

/// An event generated by a widget.
#[derive(Clone, Debug)]
pub enum WidgetEvent<APP_EVENT: Clone> {
    AppEvent(APP_EVENT),
    GainedFocus(WidgetId),
    LostFocus(WidgetId),
}
