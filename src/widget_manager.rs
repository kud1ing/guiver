use crate::{UserEvent, Widget, WidgetEvent};
use druid_shell::piet::Piet;
use druid_shell::Region;
use std::any::Any;
use std::collections::HashMap;

///
pub type WidgetId = u64;

///
pub struct WidgetManager {
    /// The IDs of child widgets.
    child_widget_ids_per_widget_id: HashMap<WidgetId, Vec<WidgetId>>,
    /// The ID of the focused widget.
    focused_widget_id: Option<WidgetId>,
    /// The counter for the next widget ID.
    next_widget_id: WidgetId,
    /// The IDs of the root widgets.
    root_widget_ids: Vec<WidgetId>,
    /// The widgets per ID.
    widgets_per_id: HashMap<WidgetId, Box<dyn Widget>>,
}

impl WidgetManager {
    pub(crate) fn new() -> Self {
        WidgetManager {
            child_widget_ids_per_widget_id: HashMap::new(),
            focused_widget_id: None,
            next_widget_id: 0,
            root_widget_ids: vec![],
            widgets_per_id: HashMap::new(),
        }
    }

    ///
    pub fn add_widget(&mut self, widget: Box<dyn Widget>) -> WidgetId {
        self.next_widget_id += 1;
        let widget_id = self.next_widget_id;

        self.root_widget_ids.push(widget_id);
        self.widgets_per_id.insert(widget_id, widget);

        widget_id
    }

    ///
    pub fn clear(&mut self) {
        self.child_widget_ids_per_widget_id.clear();
        self.focused_widget_id = None;
        self.root_widget_ids.clear();
        self.widgets_per_id.clear();
    }

    ///
    pub fn delete_widget(&mut self, widget_id: WidgetId) {
        self.widgets_per_id.remove(&widget_id);
    }

    ///
    pub(crate) fn handle_user_event(
        &mut self,
        user_event: &UserEvent,
    ) -> Vec<(WidgetId, WidgetEvent)> {
        let mut widget_ids_and_events = vec![];

        // The ID of a widget clicked by the given user event.
        let mut clicked_widget_id: Option<WidgetId> = None;

        // Iterate over all widgets.
        for (widget_id, widget) in self.widgets_per_id.iter_mut() {
            // The current widget handled the user event.
            if let Some(widget_event) = widget.handle_user_event(user_event) {
                // Collect the widget event.
                widget_ids_and_events.push((*widget_id, widget_event.clone()));

                // The widget was clicked.
                if widget_event == WidgetEvent::Clicked {
                    clicked_widget_id = Some(*widget_id)
                }
            }
        }

        // A widget was clicked by the given user event.
        if clicked_widget_id.is_some() {
            // Focus the widget clicked widget.
            self.focused_widget_id = clicked_widget_id;
            // TODO: Tell the widget that it has focus now, with `WidgetCommand::SetHasFocus`
        }
        // No widget was clicked.
        else {
            match user_event {
                // ... but the mouse went down.
                UserEvent::MouseDown(_) => {
                    // ... that means widget is focused (anymore).
                    // TODO: Tell the previously focused widget that it has lost focus, with `WidgetCommand::SetHasFocus`
                    self.focused_widget_id = None;
                }
                _ => {}
            }
        }

        widget_ids_and_events
    }

    ///
    pub(crate) fn paint(&mut self, piet: &mut Piet, region: &Region) {
        // Iterate over the root widget IDs.
        for widget_id in self.root_widget_ids.iter() {
            // Get the current root widget.
            if let Some(widget) = self.widgets_per_id.get(widget_id) {
                // Paint the current root widget.
                widget.paint(piet, region);
            }
        }
    }

    ///
    pub fn send_commands_to_widget(&mut self, widget_id: WidgetId, commands: Vec<Box<dyn Any>>) {
        // A widget with the given ID exists.
        if let Some(widget) = self.widgets_per_id.get_mut(&widget_id) {
            widget.handle_commands(commands);
            return;
        }

        // A widget with the given ID does not exist.
        // TODO: error handling?
    }
}
