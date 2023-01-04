use crate::widget_manager::WidgetsLocation;
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
