use crate::WidgetId;

///
pub struct WidgetIdProvider {
    /// The counter for the next widget ID.
    next_widget_id_counter: WidgetId,
}

impl WidgetIdProvider {
    ///
    pub fn new() -> Self {
        WidgetIdProvider {
            next_widget_id_counter: 0,
        }
    }

    ///
    pub fn next_widget_id(&mut self) -> WidgetId {
        self.next_widget_id_counter += 1;
        self.next_widget_id_counter
    }
}
