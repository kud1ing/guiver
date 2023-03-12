use crate::{HorizontalAlignment, Point, Rectangle, Size, SizeConstraints, VerticalAlignment};
use error::WidgetError;
use event::WidgetEvent;
use event_type::WidgetEventType;
use std::any::Any;

pub mod alignment;
pub mod error;
pub mod event;
pub mod event_type;
pub mod grid;
mod location;
pub mod placement;
pub mod size_constraints;
pub mod r#type;

///
pub type WidgetId = usize;

/// A trait for widgets.
/// The methods are typically called by a `WidgetManager` and parental composite widgets.
pub trait Widget<APP_EVENT: Clone> {
    /// Returns `true` if the widget generally accepts focus, like e.g. a `Button` or `TextInput`
    /// widget. A `WidgetManager` uses this to build a tab/focus order.
    fn accepts_focus(&self) -> bool {
        false
    }

    ///
    fn add_event_observation(
        &mut self,
        widget_event_type: WidgetEventType,
        widget_event: WidgetEvent<APP_EVENT>,
    );

    /// Applies the given size constraints to the widget and returns its size.
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size;

    ///
    fn event_observation(
        &mut self,
        widget_event_type: &WidgetEventType,
    ) -> Option<&WidgetEvent<APP_EVENT>>;

    /// Returns the widget's flex factor. This is used in layout widgets like `Column` and `Row`.
    fn flex_factor(&self) -> u16 {
        0
    }

    /// Returns the widget's rectangle.
    fn rectangle(&self) -> &Rectangle;

    /// Removes a container widget's child widget with the given ID.
    fn remove_child(&mut self, _child_widget_id: WidgetId) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: *self.widget_id(),
            description: "`remove_child()`".to_string(),
        })
    }

    /// Removes a container widget's child widgets.
    fn remove_children(&mut self) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: *self.widget_id(),
            description: "`remove_children()`".to_string(),
        })
    }

    ///
    fn remove_event_observation(&mut self, widget_event_type: &WidgetEventType);

    /// Returns the widget's selected value. This can be e.g. selected text in a `TextInput` widget.
    fn selected_value(&self) -> Option<Box<dyn Any>> {
        self.value()
    }

    /// Enables or disables the widget's debug rendering.
    fn set_debug_rendering(&mut self, debug_rendering: bool);

    /// Gives focus to or removes focus from the widget.
    fn set_has_focus(&mut self, _has_focus: bool) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: *self.widget_id(),
            description: "`set_has_focus()`".to_string(),
        })
    }

    /// Sets a widget's horizontal alignment. This could refer to child widgets, text etc..
    fn set_horizontal_alignment(
        &mut self,
        _horizontal_alignment: HorizontalAlignment,
    ) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: *self.widget_id(),
            description: "`set_horizontal_alignment()`".to_string(),
        })
    }

    /// Disables or enables the widget.
    fn set_is_disabled(&mut self, is_disabled: bool);

    /// Hides or shows the widget.
    fn set_is_hidden(&mut self, is_hidden: bool);

    /// Sets the widget's origin.
    fn set_origin(&mut self, origin: Point);

    /// Sets a widget's vertical alignment. This could refer to child widgets, text etc..
    fn set_vertical_alignment(
        &mut self,
        _vertical_alignment: VerticalAlignment,
    ) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: *self.widget_id(),
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
