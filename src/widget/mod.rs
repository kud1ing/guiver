mod button;
mod label;
pub mod layout;
mod placeholder;

use crate::{BoxConstraints, UserEvent};
use druid_shell::kurbo::Rect;
use druid_shell::piet;
use druid_shell::Region;
pub use label::{Label, LabelCommand};
pub use placeholder::{Placeholder, PlaceholderCommand};
use std::any::Any;

/// The commands a widget can handle.
pub enum WidgetCommand {
    SetHasFocus(bool),
    SetIsHidden(bool),
    SetRectangle(Rect),
}

///
#[derive(Clone, Debug, Eq, PartialOrd, PartialEq)]
pub enum WidgetEvent {
    Clicked,
    ValueChanged,
}

/// The requests a widget can answer.´´
pub enum WidgetRequest {
    GetRectangle,
    PreferredSize(BoxConstraints),
}

///
pub trait Widget {
    /// Handle the given commands (of types `WidgetCommand` or individual command types)
    fn handle_commands(&mut self, commands: Vec<Box<dyn Any>>);

    /// Handles the given request (of type `WidgetRequest` or an individual request type).
    fn handle_request(&mut self, request: Box<dyn Any>) -> Option<Box<dyn Any>>;

    ///
    fn handle_user_event(&mut self, event: &UserEvent) -> Option<WidgetEvent>;

    ///
    fn paint(&self, piet: &mut piet::Piet, region: &Region);
}
