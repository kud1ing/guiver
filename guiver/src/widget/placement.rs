use crate::widget::location::WidgetsLocation;
use std::any::Any;

/// Where to place a widget.
#[derive(Debug)]
pub enum WidgetPlacement {
    After(WidgetsLocation),
    Before(WidgetsLocation),
    Custom(Box<dyn Any>),
    Grid {
        column_index: usize,
        row_index: usize,
    },
}
