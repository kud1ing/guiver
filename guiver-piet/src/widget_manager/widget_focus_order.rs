use guiver::WidgetId;

///
pub(crate) struct WidgetFocusOrder {
    index_of_widget_with_focus: Option<usize>,
    widget_focus_order: Vec<WidgetId>,
}

impl WidgetFocusOrder {
    ///
    pub(crate) fn new() -> WidgetFocusOrder {
        Self {
            index_of_widget_with_focus: None,
            widget_focus_order: Vec::new(),
        }
    }

    ///
    pub(crate) fn add_widget(&mut self, widget_id: WidgetId) {
        self.widget_focus_order.push(widget_id);
        self.widget_focus_order.dedup();
    }

    ///
    pub(crate) fn focus_next_widget(&mut self) -> Option<WidgetId> {
        // A widget had focus.
        if let Some(index_of_widget_with_focus) = self.index_of_widget_with_focus {
            let mut next_index_of_widget_with_focus = index_of_widget_with_focus + 1;

            if next_index_of_widget_with_focus >= self.widget_focus_order.len() {
                next_index_of_widget_with_focus = 0;
            }

            self.index_of_widget_with_focus = Some(next_index_of_widget_with_focus);

            self.widget_focus_order
                .get(next_index_of_widget_with_focus)
                .cloned()
        }
        // No widget had focus.
        else {
            None
        }
    }

    ///
    pub(crate) fn focus_widget(&mut self, widget_id: WidgetId) {
        // A widget with that ID is in the focus order.
        if let Some(index_of_widget_with_focus) =
            self.widget_focus_order.iter().position(|&i| i == widget_id)
        {
            // Set the index of this widget.
            self.index_of_widget_with_focus = Some(index_of_widget_with_focus);
        }
        // No widget with that ID is in the focus order.
        else {
            // Focus no widget.
            self.index_of_widget_with_focus = None;
        }
    }

    ///
    pub(crate) fn remove_widget(&mut self, widget_id: WidgetId) {
        // A widget with that ID is in the focus order.
        if let Some(index_of_widget_with_remove) =
            self.widget_focus_order.iter().position(|&i| i == widget_id)
        {
            // Remove the widget.
            self.widget_focus_order.remove(index_of_widget_with_remove);

            // Adjust the focus index.
            if let Some(index_of_widget_with_focus) = self.index_of_widget_with_focus {
                if index_of_widget_with_focus >= self.widget_focus_order.len() {
                    if self.widget_focus_order.is_empty() {
                        self.index_of_widget_with_focus = None;
                    } else {
                        self.index_of_widget_with_focus = Some(0);
                    }
                }
            }
        }
    }
}
