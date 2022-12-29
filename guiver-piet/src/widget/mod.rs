mod button;
mod core;
mod hyperlink;
pub mod layout;
mod placeholder;
mod text;
mod text_input;

pub use self::core::WidgetCore;
use crate::shared_state::SharedState;
use crate::widget_manager::WidgetBox;
use crate::Event;
pub use button::Button;
use druid_shell::kurbo::{Point, Size};
use druid_shell::piet;
use druid_shell::Region;
use guiver::stroke::Stroke;
use guiver::{
    Font, HorizontalAlignment, Rect, SizeConstraints, VerticalAlignment, WidgetError, WidgetEvent,
    WidgetId, WidgetIdProvider, WidgetPlacement,
};
pub use hyperlink::Hyperlink;
use piet::PaintBrush;
pub use placeholder::Placeholder;
use std::any::Any;
pub use text::Text;
pub use text_input::TextInput;

// =================================================================================================

/// The widget trait
///
/// A widget should try to be as small as possible.
pub trait Widget {
    /// Returns `true` if the widget generally accepts focus, like e.g. a `Button` or `TextInput`
    /// widget. `WidgetManager` uses this to build a tab/focus order.
    fn accepts_focus(&self) -> bool {
        false
    }

    /// Adds the given child to a container widget.
    fn add_child(
        &mut self,
        _widget_placement: Option<WidgetPlacement>,
        _child_widget: WidgetBox,
    ) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: self.widget_id().clone(),
            description: "`add_child()`".to_string(),
        })
    }

    /// Applies the given size constraints to the widget and returns its size.
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size;

    /// Returns the widget's flex factor. This is used in layout widgets like `Column` and `Row`.
    fn flex_factor(&self) -> u16 {
        0
    }

    /// Ask the widget to handle the given event.
    fn handle_event(
        &mut self,
        widget_id_provider: &mut WidgetIdProvider,
        shared_state: &mut SharedState,
        event: &Event,
    ) -> Vec<WidgetEvent>;

    /// Paints the widget.
    fn paint(&self, piet: &mut piet::Piet, region: &Region) -> Result<(), piet::Error>;

    /// Returns the widget's rectangle.
    fn rectangle(&self) -> &Rect;

    /// Removes a container widget's child widget with the given ID.
    fn remove_child(&mut self, _child_widget_id: WidgetId) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: self.widget_id().clone(),
            description: "`remove_child()`".to_string(),
        })
    }

    /// Removes a container widget's child widgets.
    fn remove_children(&mut self) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: self.widget_id().clone(),
            description: "`remove_children()`".to_string(),
        })
    }

    /// Removes the widget's selected value. This can be e.g. selected text in a `TextInput` widget.
    fn remove_selected_value(
        &mut self,
        _widget_id_provider: &mut WidgetIdProvider,
        _shared_state: &mut SharedState,
    ) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: self.widget_id().clone(),
            description: "`remove_selected_value()`".to_string(),
        })
    }

    /// Returns the widget's selected value. This can be e.g. selected text in a `TextInput` widget.
    fn selected_value(&self) -> Option<Box<dyn Any>> {
        self.value()
    }

    /// Enables or disables the widget's debug rendering.
    fn set_debug_rendering(&mut self, debug_rendering: bool);

    /// Sets the widget's fill.
    fn set_fill(&mut self, _fill: Option<PaintBrush>) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: self.widget_id().clone(),
            description: "`set_fill()`".to_string(),
        })
    }

    /// Sets the widget's font.
    fn set_font(
        &mut self,
        _shared_state: &mut SharedState,
        _font: Font,
    ) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: self.widget_id().clone(),
            description: "`set_font()`".to_string(),
        })
    }

    /// Gives focus to or removes focus from the widget.
    fn set_has_focus(&mut self, _has_focus: bool) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: self.widget_id().clone(),
            description: "`set_has_focus()`".to_string(),
        })
    }

    /// Sets a widget's horizontal alignment. This could refer to child widgets, text etc..
    fn set_horizontal_alignment(
        &mut self,
        _horizontal_alignment: HorizontalAlignment,
    ) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: self.widget_id().clone(),
            description: "`set_horizontal_alignment()`".to_string(),
        })
    }

    /// Disables or enables the widget.
    fn set_is_disabled(&mut self, is_disabled: bool);

    /// Hides or shows the widget.
    fn set_is_hidden(&mut self, is_hidden: bool);

    /// Sets the widget's origin.
    fn set_origin(&mut self, origin: Point);

    /// Sets the widget's selected value. This can be e.g. selected text in a `TextInput` widget.
    fn set_selected_value(
        &mut self,
        _widget_id_provider: &mut WidgetIdProvider,
        _shared_state: &mut SharedState,
        _value: Box<dyn Any>,
    ) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: self.widget_id().clone(),
            description: "`set_selected_value()`".to_string(),
        })
    }

    /// Sets the widget's stroke.
    fn set_stroke(&mut self, _stroke: Option<Stroke>) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: self.widget_id().clone(),
            description: "`set_stroke()`".to_string(),
        })
    }

    /// Sets the widget's value.
    fn set_value(
        &mut self,
        _widget_id_provider: &mut WidgetIdProvider,
        _shared_state: &mut SharedState,
        _value: Box<dyn Any>,
    ) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: self.widget_id().clone(),
            description: "`srt_value()`".to_string(),
        })
    }

    /// Sets a widget's vertical alignment. This could refer to child widgets, text etc..
    fn set_vertical_alignment(
        &mut self,
        _vertical_alignment: VerticalAlignment,
    ) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: self.widget_id().clone(),
            description: "`set_vertical_alignment()`".to_string(),
        })
    }

    /// Returns the widget's value.
    fn value(&self) -> Option<Box<dyn Any>> {
        None
    }

    /// Returns the widget's ID.
    fn widget_id(&self) -> &WidgetId;
}
