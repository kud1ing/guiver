use crate::WidgetId;

///
#[derive(Debug)]
pub enum WidgetError {
    NoSuchChildWidget {
        parent_widget_id: WidgetId,
        child_widget_id: WidgetId,
    },
    NoSuchWidget(WidgetId),
    NotHandled {
        widget_id: WidgetId,
        description: String,
    },
    WidgetExistsAlready(WidgetId),
}
