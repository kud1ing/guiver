///
pub type WidgetId = usize;

///
pub trait Widget {
    /// Returns `true` if the widget generally accepts focus, like e.g. a `Button` or `TextInput`
    /// widget. A `WidgetManager` uses this to build a tab/focus order.
    fn accepts_focus(&self) -> bool {
        false
    }

    /// Returns the widget's flex factor. This is used in layout widgets like `Column` and `Row`.
    fn flex_factor(&self) -> u16 {
        0
    }

    /// Returns the widget's ID.
    fn widget_id(&self) -> &WidgetId;
}
