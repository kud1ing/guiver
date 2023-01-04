mod command;
mod widget_type;

use crate::font::Font;
use crate::stroke::Stroke;
pub use crate::widget_error::WidgetError;
pub use crate::widget_event::WidgetEvent;
pub use crate::widget_event_type::WidgetEventType;
pub use crate::widget_placement::WidgetPlacement;
pub use crate::widgets_location::WidgetsLocation;
use crate::{HorizontalAlignment, PaintBrush, Size, VerticalAlignment, WidgetId};
pub use command::Command;
use druid_shell::kurbo::Rect;
use std::any::Any;
pub use widget_type::WidgetType;

/// A widget manager decouples widgets from the business logic via `WidgetId`s and `Command`s.
///
/// In addition it can and probably should handle:
/// * a window's main widget
/// * widget focus, including tab/focus order
/// * widget event subscriptions (mapping of `WidgetEvent`s to high-level events)
/// * clipboard interaction
/// * widget styling
pub trait WidgetManager<EVENT> {
    ///
    fn handle_command(&mut self, command: Command<EVENT>) -> Result<(), WidgetError> {
        self.handle_commands(vec![command])
    }

    ///
    fn handle_commands(&mut self, commands: Vec<Command<EVENT>>) -> Result<(), WidgetError>;

    /// Returns a widget's rectangle.
    fn rectangle(&self, widget_id: WidgetId) -> Result<Rect, WidgetError>;

    ///
    fn resize(&mut self, size: Size);

    /// Returns a widget's selected value.
    fn selected_value(&self, widget_id: WidgetId) -> Result<Option<Box<dyn Any>>, WidgetError>;

    /// Returns a widget's value.
    fn value(&self, widget_id: WidgetId) -> Result<Option<Box<dyn Any>>, WidgetError>;
}
