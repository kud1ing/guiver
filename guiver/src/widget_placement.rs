use crate::widgets_location::WidgetsLocation;
use std::any::Any;

///
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
