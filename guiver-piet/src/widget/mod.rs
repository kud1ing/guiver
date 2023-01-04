mod button;
mod core;
mod hyperlink;
pub mod layout;
mod placeholder;
mod text;
mod text_input;

pub use self::core::WidgetCore;
use crate::shared_state::PietSharedState;
use crate::widget_manager::WidgetBox;
use crate::{Command, Event};
pub use button::Button;
use druid_shell::piet;
use druid_shell::Region;
use guiver::{Font, Widget, WidgetError, WidgetEvent, WidgetIdProvider, WidgetPlacement};
pub use hyperlink::Hyperlink;
pub use placeholder::Placeholder;
use std::any::Any;
pub use text::Text;
pub use text_input::TextInput;

// =================================================================================================

/// The widget trait.
///
/// A widget should try to be as small as possible.
pub trait PietWidget<APP_EVENT: Clone>: Widget<APP_EVENT> {
    /// Adds the given child to a container widget.
    fn add_child(
        &mut self,
        _widget_placement: Option<WidgetPlacement>,
        _child_widget: WidgetBox<APP_EVENT>,
    ) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: self.widget_id().clone(),
            description: "`add_child()`".to_string(),
        })
    }

    /// Ask the widget to handle the given event, possibly creating `WidgetEvent`s.
    fn handle_event(
        &mut self,
        event: &Event,
        shared_state: &mut PietSharedState,
        widget_id_provider: &mut WidgetIdProvider,
        widget_events: &mut Vec<WidgetEvent<APP_EVENT>>,
    );

    /// Paints the widget.
    fn paint(&self, piet: &mut piet::Piet, region: &Region) -> Result<(), piet::Error>;

    /// Removes the widget's selected value. This can be e.g. selected text in a `TextInput` widget.
    fn remove_selected_value(
        &mut self,
        _shared_state: &mut PietSharedState,
        _widget_id_provider: &mut WidgetIdProvider,
    ) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: self.widget_id().clone(),
            description: "`remove_selected_value()`".to_string(),
        })
    }

    /// Sets the widget's font.
    fn set_font(
        &mut self,
        _font: Font,
        _shared_state: &mut PietSharedState,
    ) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: self.widget_id().clone(),
            description: "`set_font()`".to_string(),
        })
    }

    /// Sets the widget's selected value. This can be e.g. selected text in a `TextInput` widget.
    fn set_selected_value(
        &mut self,
        _value: Box<dyn Any>,
        _shared_state: &mut PietSharedState,
        _widget_id_provider: &mut WidgetIdProvider,
    ) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: self.widget_id().clone(),
            description: "`set_selected_value()`".to_string(),
        })
    }

    /// Sets the widget's value.
    fn set_value(
        &mut self,
        _value: Box<dyn Any>,
        _shared_state: &mut PietSharedState,
        _widget_id_provider: &mut WidgetIdProvider,
        _commands: &mut Vec<Command<APP_EVENT>>,
    ) -> Result<(), WidgetError> {
        Err(WidgetError::NotHandled {
            widget_id: self.widget_id().clone(),
            description: "`set_value()`".to_string(),
        })
    }
}
