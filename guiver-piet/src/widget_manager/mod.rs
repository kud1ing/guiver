mod widget_focus_order;

use crate::shared_state::PietSharedState;
use crate::style::Style;
use crate::widget::layout::{Center, Column, Expanded, Grid, Padding, Row, SizedBox};
use crate::widget::{Button, Hyperlink, Placeholder, Text, TextInput};
use crate::widget_manager::widget_focus_order::WidgetFocusOrder;
use crate::{Event, PietWidget};
use druid_shell::kurbo::Size;
use druid_shell::piet::Piet;
use druid_shell::{piet, Clipboard, KbKey, Modifiers, Region};
use guiver::{
    Color, Command, HorizontalAlignment, Rect, SizeConstraints, WidgetError, WidgetEvent, WidgetId,
    WidgetIdProvider, WidgetManager, WidgetType,
};
use piet::PaintBrush;
use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

///
pub type PietWidgetBox<T> = Rc<RefCell<Box<dyn PietWidget<T>>>>;

/// A widget manager that uses `Piet` and `druid-shell`.
pub struct PietWidgetManager<T> {
    /// The IDs of each widget's child widgets.
    child_widget_ids_per_widget_id: HashMap<WidgetId, HashSet<WidgetId>>,
    /// The widget that has the focus.
    focused_widget: Option<PietWidgetBox<T>>,
    /// The main widget that fills the whole window.
    main_widget: Option<PietWidgetBox<T>>,
    /// The IDs of each widget's parent widget.
    parent_widget_id_per_widget_id: HashMap<WidgetId, WidgetId>,
    ///
    shared_state: PietSharedState,
    /// The size constraints. It is set in `resize()`, called by the window event handler for every
    /// window resize event so that the main widget fills the whole window.
    size_constraints: SizeConstraints,
    /// The widget style.
    style: Style,
    /// The widgets' tab/focus order.
    widget_focus_order: WidgetFocusOrder,
    /// The provide for widget IDs.
    widget_id_provider: WidgetIdProvider,
    /// All widgets per widget ID. This is used:
    /// * to determine whether a widget with a given ID exists
    /// * to pass commands to widgets directly
    widgets: HashMap<WidgetId, PietWidgetBox<T>>,
}

impl<T: Clone + 'static> PietWidgetManager<T> {
    ///
    pub fn new() -> Self {
        PietWidgetManager {
            child_widget_ids_per_widget_id: HashMap::new(),
            focused_widget: None,
            main_widget: None,
            parent_widget_id_per_widget_id: HashMap::new(),
            shared_state: PietSharedState::new(),
            size_constraints: SizeConstraints::default(),
            style: Style::default(),
            widget_focus_order: WidgetFocusOrder::new(),
            widget_id_provider: WidgetIdProvider::new(),
            widgets: HashMap::new(),
        }
    }

    ///
    /// The caller must verify that both widgets exist.
    fn add_parent_child_widget_connection(
        &mut self,
        parent_widget_id: WidgetId,
        child_widget_id: WidgetId,
    ) {
        // The child widget has a parent already already.
        if let Some(previous_parent_widget_id) =
            self.parent_widget_id_per_widget_id.get(&child_widget_id)
        {
            // Get the previous parent widget.
            let previous_parent_widget = self.widgets.get(previous_parent_widget_id).unwrap();

            // Tell the previous parent widget to remove the child widget.
            previous_parent_widget
                .borrow_mut()
                .remove_child(child_widget_id)
                .unwrap();

            // Remove the previous parent child widget connection.
            self.remove_parent_child_widget_connection(
                previous_parent_widget_id.clone(),
                child_widget_id,
            );
        }

        // Add the child to the parent.
        self.child_widget_ids_per_widget_id
            .entry(parent_widget_id)
            .or_default()
            .insert(child_widget_id);

        // Add the parent to the child.
        self.parent_widget_id_per_widget_id
            .insert(child_widget_id, parent_widget_id);
    }

    /// Puts the given widget under widget management.
    pub fn add_widget(&mut self, widget: Box<dyn PietWidget<T>>) {
        // The widget accepts focus.
        if widget.accepts_focus() {
            // Append it to the focus order.
            self.widget_focus_order
                .add_widget(widget.widget_id().clone());
        }

        self.widgets
            .insert(*widget.widget_id(), Rc::new(RefCell::new(widget)));
    }

    ///
    fn copy_selected_value_to_clipboard(&self, clipboard: &mut Clipboard) {
        // A widget has focus.
        if let Some(focused_widget) = &self.focused_widget {
            // The focused widget has a selected value.
            if let Some(selected_value) = focused_widget.borrow().selected_value() {
                // Get a string representation of the widget's selected value.
                let string_value =
                // The widget's selected value is a string.
                if let Some(string) =
                    selected_value.downcast_ref::<String>()
                {
                    string.clone()
                }
                // The widget selected value is not a string.
                else {
                    format!("{:?}", selected_value)
                };

                // Put the string value in the clipboard.
                clipboard.put_string(string_value);
            }
        }
    }

    /// Destroys the widget with the given ID and its child widget tree.
    fn destroy_widget(&mut self, widget_id: WidgetId) {
        let mut ids_of_widgets_to_destroy: HashSet<WidgetId> = HashSet::new();

        // Collect the IDs of the widget and its child widget tree.
        {
            let mut current_widget_ids: HashSet<WidgetId> = HashSet::new();
            current_widget_ids.insert(widget_id);

            while !current_widget_ids.is_empty() {
                ids_of_widgets_to_destroy.extend(&current_widget_ids);

                let mut current_child_widget_ids: HashSet<WidgetId> = HashSet::new();

                // Iterate over the current widgets.
                for current_widget_id in current_widget_ids {
                    // The current widget has child widgets.
                    if let Some(x) = self.child_widget_ids_per_widget_id.get(&current_widget_id) {
                        current_child_widget_ids.extend(x);
                    }
                }

                // Make the current widget IDs the current child widget IDs minus the already known
                // widget IDs.
                current_widget_ids = current_child_widget_ids
                    .difference(&ids_of_widgets_to_destroy)
                    .cloned()
                    .collect();
            }
        }

        // There is a focused widget.
        if let Some(focused_widget) = &self.focused_widget {
            // The focused widget is to be destroyed.
            if ids_of_widgets_to_destroy.contains(focused_widget.borrow().widget_id()) {
                self.focused_widget = None;
            }
        }

        // There is a main widget.
        if let Some(main_widget) = &self.main_widget {
            // The main widget is to be destroyed.
            if ids_of_widgets_to_destroy.contains(main_widget.borrow().widget_id()) {
                self.main_widget = None;
            }
        }

        // Iterate over the IDs of the widgets to destroy.
        for id_of_widget_to_destroy in ids_of_widgets_to_destroy {
            let child_widget_ids = self
                .child_widget_ids_per_widget_id
                .get(&id_of_widget_to_destroy)
                .cloned()
                .unwrap_or(HashSet::new());
            let parent_widget_id = self
                .parent_widget_id_per_widget_id
                .get(&id_of_widget_to_destroy)
                .cloned();

            // Remove the widget from the focus order.
            self.widget_focus_order
                .remove_widget(id_of_widget_to_destroy);

            // Remove the widget from child/parent connections.
            {
                self.child_widget_ids_per_widget_id
                    .remove(&id_of_widget_to_destroy);

                // Remove the widget from its parents' children.
                if let Some(parent_widget_id) = parent_widget_id {
                    for parents_child_widget_ids in self
                        .child_widget_ids_per_widget_id
                        .get_mut(&parent_widget_id)
                    {
                        parents_child_widget_ids.remove(&id_of_widget_to_destroy);
                    }
                }
            }

            // Remove the widget from parent/child connections.
            {
                self.parent_widget_id_per_widget_id
                    .remove(&id_of_widget_to_destroy);

                // Remove the widget from its children's parents.
                for child_widget_id in child_widget_ids {
                    self.parent_widget_id_per_widget_id.remove(&child_widget_id);
                }
            }

            self.widgets.remove(&id_of_widget_to_destroy);
        }
    }

    ///
    fn give_next_widget_focus(&mut self) -> Result<(), WidgetError> {
        // There is a next widget in the focus order.
        if let Some(widget_id_to_give_focus) = self.widget_focus_order.focus_next_widget() {
            let widget_to_give_focus = self.widgets.get(&widget_id_to_give_focus).unwrap();

            // A widget has focus already.
            if let Some(focused_widget) = &self.focused_widget {
                // The focused widget is already the next widget in the focus order.
                if focused_widget.borrow().widget_id() == &widget_id_to_give_focus {
                    return Ok(());
                }

                // Remove the focus from the previously focused widget.
                focused_widget.borrow_mut().set_has_focus(false)?;
            }

            // Give the widget focus.
            widget_to_give_focus.borrow_mut().set_has_focus(true)?;
            self.focused_widget = Some(widget_to_give_focus.clone());
        }

        Ok(())
    }

    ///
    pub fn handle_event(
        &mut self,
        event: &Event,
        clipboard: Option<&mut Clipboard>,
    ) -> Result<Vec<WidgetEvent<T>>, WidgetError> {
        let mut widget_events = vec![];

        // Handle key events.
        match event {
            Event::KeyDown(key_event) => {
                // The Meta key is pressed.
                if key_event.mods.contains(Modifiers::META) {
                    // A clipboard is given.
                    if let Some(clipboard) = clipboard {
                        // Handle paste from clipboard.
                        if key_event.key == KbKey::Character("v".to_string()) {
                            // Could get a string from the clipboard.
                            if let Some(string) = clipboard.get_string() {
                                // A widget has focus.
                                if let Some(focused_widget) = &mut self.focused_widget {
                                    // Let the focused widget handle a clipboard paste event.
                                    focused_widget.borrow_mut().handle_event(
                                        &Event::ClipboardPaste(string),
                                        &mut self.shared_state,
                                        &mut self.widget_id_provider,
                                        &mut widget_events,
                                    );
                                }
                            }
                        }
                        // Handle copy to clipboard.
                        else if key_event.key == KbKey::Character("c".to_string()) {
                            self.copy_selected_value_to_clipboard(clipboard);
                        }
                        // Handle cut to clipboard.
                        else if key_event.key == KbKey::Character("x".to_string()) {
                            self.copy_selected_value_to_clipboard(clipboard);

                            // A widget has focus.
                            if let Some(focused_widget) = &mut self.focused_widget {
                                // Remove the selected value.
                                focused_widget.borrow_mut().remove_selected_value(
                                    &mut self.shared_state,
                                    &mut self.widget_id_provider,
                                )?;
                            }
                        }
                    }
                }
                // The Meta key is not pressed.
                else {
                    // Tab was pressed.
                    if key_event.key == KbKey::Tab {
                        // Give the next widget focus.
                        self.give_next_widget_focus()?;
                    }
                    // Tab was not pressed.
                    else {
                        // A widget has focus.
                        if let Some(focused_widget) = &mut self.focused_widget {
                            // Let the focused widget handle the key event.
                            focused_widget.borrow_mut().handle_event(
                                event,
                                &mut self.shared_state,
                                &mut self.widget_id_provider,
                                &mut widget_events,
                            );
                        }
                    }
                }

                return Ok(widget_events);
            }
            Event::KeyUp(_key_event) => {
                // A widget has focus.
                if let Some(focused_widget) = &mut self.focused_widget {
                    // Let the focused widget handle the key event.
                    focused_widget.borrow_mut().handle_event(
                        event,
                        &mut self.shared_state,
                        &mut self.widget_id_provider,
                        &mut widget_events,
                    );
                }

                return Ok(widget_events);
            }
            _ => {}
        }

        // There is a main widget.
        if let Some(main_widget) = &mut self.main_widget {
            // Let the main widget handle the given user event.
            main_widget.borrow_mut().handle_event(
                event,
                &mut self.shared_state,
                &mut self.widget_id_provider,
                &mut widget_events,
            );
        }

        // Focus handling.
        {
            let mut id_of_the_last_widget_that_gained_focus = None;

            // Iterate over the widget events in search of focus events.
            for widget_event in &widget_events {
                match widget_event {
                    WidgetEvent::GainedFocus(widget_id) => {
                        // A widget gained focus.
                        id_of_the_last_widget_that_gained_focus = Some(widget_id.clone());
                    }
                    WidgetEvent::LostFocus(widget_id) => {
                        // A widget had focus.
                        if let Some(focused_widget) = &mut self.focused_widget {
                            // The widget that lost focus had focus before.
                            if focused_widget.borrow().widget_id() == widget_id {
                                self.focused_widget = None;
                            }
                        }
                    }
                    _ => {}
                }
            }

            // A widget gained focus.
            if let Some(id_of_the_widget_that_gained_focus) =
                id_of_the_last_widget_that_gained_focus
            {
                // There is a widget with the given ID.
                if let Some(widget_box) = self.widgets.get(&id_of_the_widget_that_gained_focus) {
                    // A widget had focus.
                    if let Some(focused_widget) = &mut self.focused_widget {
                        // The widgets are different.
                        if focused_widget.borrow().widget_id()
                            != &id_of_the_widget_that_gained_focus
                        {
                            // Unfocus that previously focused widget.
                            focused_widget.borrow_mut().set_has_focus(false)?;
                        }
                    }

                    // Remember the current widget as focused.
                    self.focused_widget = Some(widget_box.clone());
                }
                // There is no widget with the given ID.
                else {
                    return Err(WidgetError::NoSuchWidget(
                        id_of_the_widget_that_gained_focus,
                    ));
                };
            }
        }

        Ok(widget_events)
    }

    ///
    pub fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), piet::Error> {
        // There is a main widget.
        if let Some(main_widget) = &self.main_widget {
            // Paint the main widget.
            main_widget.borrow().paint(piet, region)?;
        }

        Ok(())
    }

    ///
    fn remove_parent_child_widget_connection(
        &mut self,
        parent_widget_id: WidgetId,
        child_widget_id: WidgetId,
    ) {
        // Remove the child widget from the focus order.
        self.widget_focus_order.remove_widget(child_widget_id);

        self.child_widget_ids_per_widget_id
            .entry(parent_widget_id)
            .or_default()
            .remove(&child_widget_id);

        self.parent_widget_id_per_widget_id.remove(&child_widget_id);
    }

    ///
    fn remove_parent_child_widget_connections(&mut self, parent_widget_id: WidgetId) {
        // Iterate over and remove the child widget IDs for the given parent widget ID.
        for child_widget_id in self
            .child_widget_ids_per_widget_id
            .entry(parent_widget_id)
            .or_default()
            .drain()
        {
            // Remove the child widget from the focus order.
            self.widget_focus_order.remove_widget(child_widget_id);

            // Remove the parent widget ID for the current child widget ID.
            self.parent_widget_id_per_widget_id.remove(&child_widget_id);
        }
    }

    ///
    fn widget(&self, widget_id: WidgetId) -> Result<&PietWidgetBox<T>, WidgetError> {
        // There is a widget with the given ID.
        if let Some(widget_box) = self.widgets.get(&widget_id) {
            Ok(widget_box)
        }
        // There is no widget with the given ID.
        else {
            Err(WidgetError::NoSuchWidget(widget_id))
        }
    }

    ///
    pub fn widget_id_provider(&mut self) -> &mut WidgetIdProvider {
        &mut self.widget_id_provider
    }

    ///
    fn widget_mut(&mut self, widget_id: WidgetId) -> Result<&mut PietWidgetBox<T>, WidgetError> {
        // There is a widget with the given ID.
        if let Some(widget_box) = self.widgets.get_mut(&widget_id) {
            Ok(widget_box)
        }
        // There is no widget with the given ID.
        else {
            Err(WidgetError::NoSuchWidget(widget_id))
        }
    }
}

impl<T: Clone + 'static> WidgetManager<T> for PietWidgetManager<T> {
    fn handle_commands(&mut self, mut commands: Vec<Command<T>>) -> Result<(), WidgetError> {
        loop {
            let mut next_commands = vec![];

            // Iterate over the given commands.
            for command in commands {
                // Get the ID of the widget from the command.
                let widget_id = *command.widget_id();

                match command {
                    Command::AddChild {
                        widget_placement,
                        child_widget_id,
                        ..
                    } => {
                        let widget_box = self.widget(widget_id)?;

                        // There is a widget with the child widget ID from the command.
                        let child_widget_box =
                            if let Some(child_widget_box) = self.widgets.get(&child_widget_id) {
                                child_widget_box
                            }
                            // There is no widget with the given child ID.
                            else {
                                return Err(WidgetError::NoSuchWidget(child_widget_id));
                            };

                        widget_box
                            .borrow_mut()
                            .add_child(widget_placement, child_widget_box.clone())?;

                        self.add_parent_child_widget_connection(widget_id, child_widget_id);
                    }
                    Command::AddChildren { child_widgets, .. } => {
                        // Iterate over the child widgets. This additional loop is in order to escape
                        // the borrow checker.
                        for (_, child_widget_id) in &child_widgets {
                            self.add_parent_child_widget_connection(widget_id, *child_widget_id);
                        }

                        let widget_box = self.widget(widget_id)?;

                        // Iterate over the child widgets.
                        for (widget_placement, child_widget_id) in child_widgets {
                            // There is a widget with the child widget ID from the command.
                            let child_widget_box = if let Some(child_widget_box) =
                                self.widgets.get(&child_widget_id)
                            {
                                child_widget_box
                            }
                            // There is no widget with the given child ID.
                            else {
                                return Err(WidgetError::NoSuchWidget(child_widget_id));
                            };

                            widget_box
                                .borrow_mut()
                                .add_child(widget_placement, child_widget_box.clone())?;
                        }
                    }
                    Command::AddEventObservation(_widget_id, widget_event_type, custom_value) => {
                        let widget_box = self.widget(widget_id)?;

                        widget_box.borrow_mut().add_event_observation(
                            widget_event_type.clone(),
                            WidgetEvent::Custom(custom_value),
                        );
                    }
                    Command::CreateWidget(widget_id, widget_type) => {
                        // A widget with the given ID exists already.
                        if self.widgets.contains_key(&widget_id) {
                            return Err(WidgetError::WidgetExistsAlready(widget_id));
                        }

                        let widget_box: Box<dyn PietWidget<T>> = match widget_type {
                            WidgetType::Center => Box::new(Center::new(
                                widget_id,
                                self.style.debug_rendering_stroke.clone(),
                            )),
                            WidgetType::Column => Box::new(Column::new(
                                widget_id,
                                self.style.debug_rendering_stroke.clone(),
                                HorizontalAlignment::Center,
                                self.style.spacing,
                            )),
                            WidgetType::Expanded { flex_factor } => Box::new(Expanded::new(
                                widget_id,
                                self.style.debug_rendering_stroke.clone(),
                                flex_factor,
                            )),
                            WidgetType::Grid {
                                column_properties,
                                row_properties,
                            } => Box::new(Grid::new(
                                widget_id,
                                self.style.debug_rendering_stroke.clone(),
                                column_properties,
                                row_properties,
                            )),
                            WidgetType::Hyperlink(text) => {
                                let mut font_unvisited = self.style.font.clone();
                                font_unvisited.font_color = Color::rgb8(100, 100, 255);

                                let mut font_being_clicked = self.style.font.clone();
                                font_being_clicked.font_color = self.style.accent_color.clone();

                                let mut font_visited = self.style.font.clone();
                                font_visited.font_color = Color::rgb8(50, 50, 100);

                                Box::new(Hyperlink::new(
                                    widget_id,
                                    self.style.debug_rendering_stroke.clone(),
                                    self.shared_state.piet_text(),
                                    font_unvisited,
                                    font_being_clicked,
                                    font_visited,
                                    text,
                                ))
                            }
                            WidgetType::Padding => Box::new(Padding::new(
                                widget_id,
                                self.style.debug_rendering_stroke.clone(),
                                self.style.padding,
                                self.style.padding,
                                self.style.padding,
                                self.style.padding,
                            )),
                            WidgetType::Placeholder { maximum_size } => Box::new(Placeholder::new(
                                widget_id,
                                self.style.debug_rendering_stroke.clone(),
                                maximum_size,
                            )),
                            WidgetType::Row => Box::new(Row::new(
                                widget_id,
                                self.style.debug_rendering_stroke.clone(),
                                self.style.vertical_alignment,
                                self.style.spacing,
                            )),
                            WidgetType::SizedBox { desired_size } => Box::new(SizedBox::new(
                                widget_id,
                                self.style.debug_rendering_stroke.clone(),
                                desired_size,
                            )),
                            WidgetType::Text(text) => Box::new(Text::new(
                                widget_id,
                                self.style.debug_rendering_stroke.clone(),
                                self.shared_state.piet_text(),
                                self.style.font.clone(),
                                text,
                            )),
                            WidgetType::TextButton(text) => {
                                let child_widget = Text::new(
                                    self.widget_id_provider.next_widget_id(),
                                    self.style.debug_rendering_stroke.clone(),
                                    self.shared_state.piet_text(),
                                    self.style.font.clone(),
                                    text,
                                );

                                Box::new(Button::new(
                                    widget_id,
                                    self.style.debug_rendering_stroke.clone(),
                                    Rc::new(RefCell::new(Box::new(child_widget))),
                                    Some(PaintBrush::Color(self.style.accent_color.clone())),
                                    Some(self.style.frame_color.clone()),
                                    Some(self.style.accent_color.clone()),
                                ))
                            }
                            WidgetType::TextInput { text, width } => Box::new(TextInput::new(
                                widget_id,
                                self.style.debug_rendering_stroke.clone(),
                                self.shared_state.piet_text(),
                                self.style.font.clone(),
                                text,
                                width,
                                self.style.frame_color.clone(),
                                self.style.accent_color.clone(),
                            )),
                        };

                        self.add_widget(widget_box);
                    }
                    Command::Destroy(_widget_id) => self.destroy_widget(widget_id),
                    Command::RemoveChild {
                        parent_widget_id,
                        child_widget_id,
                        destroy_child_widget: destroy,
                    } => {
                        let widget_box = self.widget(widget_id)?;
                        widget_box.borrow_mut().remove_child(child_widget_id)?;

                        // Destroy the child widget.
                        if destroy {
                            self.destroy_widget(parent_widget_id);
                        }
                        // Remove the child widget.
                        else {
                            self.remove_parent_child_widget_connection(
                                parent_widget_id,
                                child_widget_id,
                            );
                        }
                    }
                    Command::RemoveChildren {
                        destroy_child_widgets,
                        ..
                    } => {
                        let widget_box = self.widget(widget_id)?;
                        widget_box.borrow_mut().remove_children()?;

                        // Destroy the child widgets.
                        if destroy_child_widgets {
                            // Get the widget's the child widget IDs.
                            let child_widget_ids = self
                                .child_widget_ids_per_widget_id
                                .entry(widget_id)
                                .or_default()
                                .clone();

                            // Iterate over the child widget IDs.
                            for child_widget_id in child_widget_ids {
                                // Destroy the child widget.
                                self.destroy_widget(child_widget_id);
                            }
                        }
                        // Remove the child widgets.
                        else {
                            self.remove_parent_child_widget_connections(widget_id);
                        }
                    }
                    Command::RemoveEventObservation(_widget_id, widget_event_type) => {
                        let widget_box = self.widget(widget_id)?;

                        widget_box
                            .borrow_mut()
                            .remove_event_observation(&widget_event_type);
                    }
                    Command::SetDebugRendering(_widget_id, debug_rendering) => {
                        let widget_box = self.widget(widget_id)?;
                        widget_box.borrow_mut().set_debug_rendering(debug_rendering);
                    }
                    Command::SetFill(_widget_id, fill) => {
                        let widget_box = self.widget(widget_id)?;
                        widget_box.borrow_mut().set_fill(fill)?;
                    }
                    Command::SetFont(_widget_id, font) => {
                        // There is a widget with the given ID.
                        let widget_box = if let Some(widget_box) = self.widgets.get(&widget_id) {
                            widget_box
                        }
                        // There is no widget with the given ID.
                        else {
                            return Err(WidgetError::NoSuchWidget(widget_id));
                        };

                        widget_box
                            .borrow_mut()
                            .set_font(font, &mut self.shared_state)?;
                    }
                    Command::SetHasFocus(_widget_id, has_focus) => {
                        // There is a widget with the given ID.
                        let widget_box = if let Some(widget_box) = self.widgets.get(&widget_id) {
                            widget_box
                        }
                        // There is no widget with the given ID.
                        else {
                            return Err(WidgetError::NoSuchWidget(widget_id));
                        };

                        let mut widget_had_focus_already = false;

                        // A widget had focus.
                        if let Some(focused_widget) = &mut self.focused_widget {
                            // The widgets are different.
                            if *focused_widget.borrow().widget_id() != widget_id {
                                // Unfocus that widget.
                                focused_widget.borrow_mut().set_has_focus(false)?;
                            }
                            // The widgets are the same.
                            else {
                                widget_had_focus_already = true;
                            }
                        }

                        if !widget_had_focus_already {
                            // Tell the widget it has focus.
                            widget_box.borrow_mut().set_has_focus(has_focus)?;

                            // Select the widget in the focus order.
                            self.widget_focus_order.focus_widget(widget_id);

                            // Remember the current widget as focused.
                            self.focused_widget = Some(widget_box.clone());
                        }
                    }
                    Command::SetHorizontalAlignment(_widget_id, horizontal_alignment) => {
                        let widget_box = self.widget(widget_id)?;
                        widget_box
                            .borrow_mut()
                            .set_horizontal_alignment(horizontal_alignment)?;
                    }
                    Command::SetIsDisabled(_widget_id, is_disabled) => {
                        let widget_box = self.widget(widget_id)?;
                        widget_box.borrow_mut().set_is_disabled(is_disabled);
                    }
                    Command::SetIsHidden(_widget_id, is_hidden) => {
                        let widget_box = self.widget(widget_id)?;
                        widget_box.borrow_mut().set_is_hidden(is_hidden);
                    }
                    Command::SetMainWidget(_widget_id) => {
                        let widget_box = self.widget(widget_id)?;
                        widget_box.borrow_mut().set_origin((1.0, 1.0).into());
                        self.main_widget = Some(widget_box.clone());
                    }
                    Command::SetStroke(_widget_id, stroke) => {
                        let widget_box = self.widget(widget_id)?;
                        widget_box.borrow_mut().set_stroke(stroke)?;
                    }
                    Command::SetValue(_widget_id, value) => {
                        // There is a widget with the given ID.
                        let widget_box = if let Some(widget_box) = self.widgets.get(&widget_id) {
                            widget_box
                        }
                        // There is no widget with the given ID.
                        else {
                            return Err(WidgetError::NoSuchWidget(widget_id));
                        };

                        widget_box.borrow_mut().set_value(
                            value,
                            &mut self.shared_state,
                            &mut self.widget_id_provider,
                            &mut next_commands,
                        )?;
                    }
                    Command::SetVerticalAlignment(_widget_id, vertical_alignment) => {
                        let widget_box = self.widget(widget_id)?;
                        widget_box
                            .borrow_mut()
                            .set_vertical_alignment(vertical_alignment)?;
                    }
                };
            }

            if next_commands.is_empty() {
                break;
            }

            commands = next_commands;
        }

        // There is a main widget.
        if let Some(main_widget) = &mut self.main_widget {
            // The widget commands might have affected the layout. Apply the size constraints again
            // for re-layout.
            main_widget
                .borrow_mut()
                .apply_size_constraints(self.size_constraints);
        }

        Ok(())
    }

    /// Returns a widget's rectangle.
    fn rectangle(&self, widget_id: WidgetId) -> Result<Rect, WidgetError> {
        Ok(self.widget(widget_id)?.borrow().rectangle().clone())
    }

    fn resize(&mut self, size: Size) {
        // Create a new size constraint from the given window size.
        let size_constraints = SizeConstraints::tight(size - Size::new(2.0, 2.0));

        // Use the new size constraint.
        self.size_constraints = size_constraints;

        // There is a main widget.
        if let Some(main_widget) = &mut self.main_widget {
            // Resize the main widget.
            main_widget
                .borrow_mut()
                .apply_size_constraints(size_constraints);
        }
    }

    fn selected_value(&self, widget_id: WidgetId) -> Result<Option<Box<dyn Any>>, WidgetError> {
        Ok(self.widget(widget_id)?.borrow().selected_value())
    }

    fn value(&self, widget_id: WidgetId) -> Result<Option<Box<dyn Any>>, WidgetError> {
        Ok(self.widget(widget_id)?.borrow().value())
    }
}
