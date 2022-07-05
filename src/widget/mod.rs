mod button;
mod label;
pub mod layout;
mod placeholder;
pub mod size_constraints;

use crate::{SizeConstraints, UserEvent};
use druid_shell::kurbo::{Point, Size};
use druid_shell::piet;
use druid_shell::Region;
pub use label::Label;
pub use placeholder::Placeholder;
use std::any::Any;

pub type WidgetId = u64;

///
#[derive(Debug)]
pub enum WidgetCommand {
    Remove(WidgetId),
    SetHasFocus(WidgetId, bool),
    SetIsDisabled(WidgetId, bool),
    SetIsHidden(WidgetId, bool),
    SetValue(WidgetId, Box<dyn Any>),
}

///
#[derive(Clone, Debug)]
pub enum WidgetEvent {
    Clicked(WidgetId),
    ValueChanged(WidgetId),
}

///
pub trait Widget {
    ///
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size;

    ///
    fn handle_user_event(&mut self, event: &UserEvent, widget_events: &mut Vec<WidgetEvent>);

    ///
    fn handle_widget_command(&mut self, widget_command: &WidgetCommand);

    ///
    fn paint(&self, piet: &mut piet::Piet, region: &Region);

    ///
    fn set_has_focus(&mut self, has_focus: bool);

    ///
    fn set_is_hidden(&mut self, is_hidden: bool);

    ///
    fn set_origin(&mut self, origin: Point);

    ///
    fn widget_id(&self) -> &WidgetId;
}
