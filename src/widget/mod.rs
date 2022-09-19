mod button;
mod core;
mod hyperlink;
pub mod layout;
mod placeholder;
mod text;
mod text_input;

use crate::stroke::Stroke;
use crate::widget_manager::WidgetBox;
use crate::{Event, Font, HorizontalAlignment, Rect, SizeConstraints, VerticalAlignment};
pub use button::Button;
use druid_shell::kurbo::{Point, Size};
use druid_shell::piet;
use druid_shell::Region;
pub use hyperlink::Hyperlink;
use piet::PaintBrush;
pub use placeholder::Placeholder;
use std::any::Any;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
pub use text::Text;
pub use text_input::TextInput;

///
pub type WidgetId = usize;

/// A command to a widget.
pub enum WidgetCommand {
    /// Append the child widget.
    AppendChild(WidgetBox),
    /// Remove the widget's children.
    RemoveAllChildren,
    /// Remove the child widget.
    RemoveChild(WidgetId),
    /// Enables/disables debug rendering mode.
    SetDebugRendering(bool),
    /// Sets/unsets the widget's fill.
    SetFill(Option<PaintBrush>),
    /// Sets the widget's font.
    SetFont(Font),
    /// Gives/removes focus to the widget.
    SetHasFocus(bool),
    /// Sets the widget's horizontal alignment.
    SetHorizontalAlignment(HorizontalAlignment),
    /// Enables/disables the widget.
    SetIsDisabled(bool),
    /// Hides/shows the widget.
    SetIsHidden(bool),
    /// Sets/unsets the widget's stroke.
    SetStroke(Option<Stroke>),
    /// Sets the given value to the widget.
    SetValue(Box<dyn Any>),
    /// Sets the widget's vertical alignment.
    SetVerticalAlignment(VerticalAlignment),
}

impl Debug for WidgetCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WidgetCommand::AppendChild(widget_box) => {
                write!(
                    f,
                    "WidgetCommand::AppendChild({:?})",
                    RefCell::borrow(widget_box).widget_id()
                )
            }
            WidgetCommand::RemoveAllChildren => {
                write!(f, "WidgetCommand::RemoveAllChildren")
            }
            WidgetCommand::RemoveChild(_) => {
                write!(f, "WidgetCommand::RemoveChild(...)")
            }
            WidgetCommand::SetDebugRendering(_) => {
                write!(f, "WidgetCommand::SetDebugRendering(...)")
            }
            WidgetCommand::SetFill(_) => {
                write!(f, "WidgetCommand::SetFill(...)")
            }
            WidgetCommand::SetFont(_) => {
                write!(f, "WidgetCommand::SetFont(...)")
            }
            WidgetCommand::SetHasFocus(_) => {
                write!(f, "WidgetCommand::SetHasFocus(...)")
            }
            WidgetCommand::SetHorizontalAlignment(_) => {
                write!(f, "WidgetCommand::SetHorizontalAlignment(...)")
            }
            WidgetCommand::SetIsDisabled(_) => {
                write!(f, "WidgetCommand::SetIsDisabled(...)")
            }
            WidgetCommand::SetIsHidden(_) => {
                write!(f, "WidgetCommand::SetIsHidden(...)")
            }
            WidgetCommand::SetStroke(_) => {
                write!(f, "WidgetCommand::SetStroke(...)")
            }
            WidgetCommand::SetValue(_) => {
                write!(f, "WidgetCommand::SetValue(...)")
            }
            WidgetCommand::SetVerticalAlignment(_) => {
                write!(f, "WidgetCommand::SetVerticalAlignment(...)")
            }
        }
    }
}

// =================================================================================================

///
#[derive(Debug)]
pub enum WidgetError {
    CommandNotHandled(WidgetId, WidgetCommand),
    NoSuchWidget(WidgetId),
}

// =================================================================================================

/// An event generated by a widget.
#[derive(Debug)]
pub enum WidgetEvent {
    /// The widget was clicked.
    Clicked(WidgetId),
    /// The widget gained focus.
    GainedFocus(WidgetId),
    /// The widget lost focus.
    LostFocus(WidgetId),
    /// The widget selected value was changed.
    SelectedValueChanged(WidgetId, Box<dyn Any>),
    /// The widget was submitted, e.g. by pressing Enter on a text input.
    Submitted(WidgetId),
    /// The widget value was changed.
    ValueChanged(WidgetId, Box<dyn Any>),
}

///
pub trait Widget {
    ///
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size;

    ///
    fn flex_factor(&self) -> u16 {
        0
    }

    ///
    fn handle_command(&mut self, widget_command: WidgetCommand) -> Result<(), WidgetError>;

    ///
    fn handle_event(&mut self, event: &Event, widget_events: &mut Vec<WidgetEvent>);

    ///
    fn paint(&self, piet: &mut piet::Piet, region: &Region) -> Result<(), piet::Error>;

    ///
    fn rectangle(&self) -> &Rect;

    ///
    fn set_origin(&mut self, origin: Point);

    ///
    fn widget_id(&self) -> &WidgetId;
}
