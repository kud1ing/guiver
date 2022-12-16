use crate::stroke::Stroke;
use crate::style::Style;
use crate::widget::layout::{
    Center, Column, Expanded, Grid, GridColumnProperties, GridRowProperties, Padding, Row, SizedBox,
};
use crate::widget::{
    Button, Hyperlink, Placeholder, Text, TextInput, WidgetError, WidgetPlacement,
};
use crate::{
    Color, Event, Font, HorizontalAlignment, SizeConstraints, VerticalAlignment, Widget,
    WidgetEvent, WidgetId,
};
use druid_shell::kurbo::Size;
use druid_shell::piet::Piet;
use druid_shell::{piet, Clipboard, KbKey, Modifiers, Region};
use piet::PaintBrush;
use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

///
pub type WidgetBox = Rc<RefCell<Box<dyn Widget>>>;

/// A command to the widget manager or widgets.
#[derive(Debug)]
pub enum Command {
    /// Adds the child widget with the given ID to the parent widget.
    AddChild {
        parent_widget_id: WidgetId,
        widget_placement: Option<WidgetPlacement>,
        child_widget_id: WidgetId,
    },
    /// Destroys the widget with the given ID.
    Destroy(WidgetId),
    /// Removes the child widget with the given ID from the parent widget.
    RemoveChild {
        parent_widget_id: WidgetId,
        child_widget_id: WidgetId,
        destroy_child_widget: bool,
    },
    /// Removes the widget's child widgets.
    RemoveChildren {
        parent_widget_id: WidgetId,
        destroy_child_widgets: bool,
    },
    /// Sets the child widget at the given location.
    SetChild {
        parent_widget_id: WidgetId,
        column_index: usize,
        row_index: usize,
        child_widget_id: WidgetId,
    },
    /// Enables/disables debug rendering mode for the widget.
    SetDebugRendering(WidgetId, bool),
    /// Sets/unsets the widget's fill.
    SetFill(WidgetId, Option<PaintBrush>),
    /// Sets the widget's font.
    SetFont(WidgetId, Font),
    /// Gives/removes focus to the widget.
    SetHasFocus(WidgetId, bool),
    /// Sets the widget's horizontal alignment.
    SetHorizontalAlignment(WidgetId, HorizontalAlignment),
    /// Enables/disables the widget.
    SetIsDisabled(WidgetId, bool),
    /// Hides/shows the widget.
    SetIsHidden(WidgetId, bool),
    /// Makes the widget with the given ID the main widget.
    SetMainWidget(WidgetId),
    /// Sets/unsets the widget's stroke.
    SetStroke(WidgetId, Option<Stroke>),
    /// Sets the given value to the widget.
    SetValue(WidgetId, Box<dyn Any>),
    /// Sets the widget's vertical alignment.
    SetVerticalAlignment(WidgetId, VerticalAlignment),
}

impl Command {
    /// Returns the ID of the receiver widget.
    pub fn widget_id(&self) -> &WidgetId {
        match self {
            Command::AddChild {
                parent_widget_id, ..
            } => parent_widget_id,
            Command::Destroy(widget_id) => widget_id,
            Command::RemoveChild {
                parent_widget_id, ..
            } => parent_widget_id,
            Command::RemoveChildren {
                parent_widget_id, ..
            } => parent_widget_id,
            Command::SetChild {
                parent_widget_id, ..
            } => parent_widget_id,
            Command::SetDebugRendering(widget_id, _) => widget_id,
            Command::SetFill(widget_id, _) => widget_id,
            Command::SetFont(widget_id, _) => widget_id,
            Command::SetHasFocus(widget_id, _) => widget_id,
            Command::SetHorizontalAlignment(widget_id, _) => widget_id,
            Command::SetIsDisabled(widget_id, _) => widget_id,
            Command::SetIsHidden(widget_id, _) => widget_id,
            Command::SetMainWidget(widget_id) => widget_id,
            Command::SetStroke(widget_id, _) => widget_id,
            Command::SetValue(widget_id, _) => widget_id,
            Command::SetVerticalAlignment(widget_id, _) => widget_id,
        }
    }
}

// =================================================================================================

///
pub struct WidgetManager {
    /// The IDs of each widget's child widgets.
    child_widget_ids_per_widget_id: HashMap<WidgetId, HashSet<WidgetId>>,
    /// The widget that has the focus.
    focused_widget: Option<WidgetBox>,
    /// The main widget that fills the whole window.
    main_widget: Option<WidgetBox>,
    /// The counter for the next widget ID.
    next_widget_id_counter: WidgetId,
    /// The IDs of each widget's parent widget.
    parent_widget_id_per_widget_id: HashMap<WidgetId, WidgetId>,
    /// The size constraints. It is set in `resize()`, called by the window event handler for every
    /// window resize event so that the main widget fills the whole window.
    size_constraints: SizeConstraints,
    ///
    style: Style,
    /// All widgets per widget ID. This is used:
    /// * to determine whether a widget with a given ID exists
    /// * to pass commands to a widget
    widgets: HashMap<WidgetId, WidgetBox>,
}

impl WidgetManager {
    ///
    pub fn new() -> Self {
        WidgetManager {
            child_widget_ids_per_widget_id: HashMap::new(),
            focused_widget: None,
            main_widget: None,
            next_widget_id_counter: 0,
            parent_widget_id_per_widget_id: HashMap::new(),
            size_constraints: SizeConstraints::default(),
            style: Style::default(),
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
    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        // TODO: Append the widget to the tab order if it accepts focus.

        self.widgets
            .insert(*widget.widget_id(), Rc::new(RefCell::new(widget)));
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

            // Remove the current widget to destroy from `child_widget_ids_per_widget_id`.
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

            // Remove the current widget to destroy from `parent_widget_ids_per_widget_id`.
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
    pub fn handle_event(
        &mut self,
        event: &Event,
        clipboard: Option<&mut Clipboard>,
    ) -> Result<Vec<WidgetEvent>, WidgetError> {
        let mut widget_events = vec![];

        // Handle key events.
        match event {
            Event::KeyDown(key_event) => {
                // A widget has focus.
                if let Some(focused_widget) = &mut self.focused_widget {
                    // The Meta key is pressed.
                    if key_event.mods.contains(Modifiers::META) {
                        // A clipboard is given.
                        if let Some(clipboard) = clipboard {
                            // Handle paste from clipboard.
                            if key_event.key == KbKey::Character("v".to_string()) {
                                // Could get a string from the clipboard.
                                if let Some(string) = clipboard.get_string() {
                                    // Let the focused widget handle a clipboard past event.
                                    focused_widget.borrow_mut().handle_event(
                                        &Event::ClipboardPaste(string),
                                        &mut widget_events,
                                    );
                                }
                            }
                            // Handle copy to clipboard.
                            else if key_event.key == KbKey::Character("c".to_string()) {
                                // The focused widget's has a value.
                                if let Some(widget_value) = focused_widget.borrow().selected_value()
                                {
                                    // The widget value is a string.
                                    let string_value = if let Some(string) =
                                        widget_value.downcast_ref::<String>()
                                    {
                                        string.clone()
                                    }
                                    // The widget value is not a string.
                                    else {
                                        format!("{:?}", widget_value)
                                    };

                                    // Put the value in the clipboard.
                                    clipboard.put_string(string_value);
                                }
                            }
                            // Handle cut to clipboard.
                            else if key_event.key == KbKey::Character("x".to_string()) {
                                // TODO
                                println!("TODO: cut");
                            }
                        }
                    }
                    // The Meta key is not pressed.
                    else {
                        // Let the focused widget handle the key event.
                        focused_widget
                            .borrow_mut()
                            .handle_event(event, &mut widget_events);
                    }
                }

                return Ok(widget_events);
            }
            Event::KeyUp(_key_event) => {
                // A widget has focus.
                if let Some(focused_widget) = &mut self.focused_widget {
                    // Let the focused widget handle the key event.
                    focused_widget
                        .borrow_mut()
                        .handle_event(event, &mut widget_events);
                }

                return Ok(widget_events);
            }
            _ => {}
        }

        // There is a main widget.
        if let Some(main_widget) = &mut self.main_widget {
            // Let the main widget handle the given user event.
            main_widget
                .borrow_mut()
                .handle_event(event, &mut widget_events);
        }

        // Focus handling.
        {
            let mut id_of_the_last_widget_that_gained_focus = None;

            // Iterate over the widget events in search of focus events.
            for widget_event in &widget_events {
                match widget_event {
                    WidgetEvent::GainedFocus(widget_id) => {
                        // A widget gained focus.
                        id_of_the_last_widget_that_gained_focus = Some(widget_id);
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
                if let Some(widget_box) = self.widgets.get(id_of_the_widget_that_gained_focus) {
                    // A widget had focus.
                    if let Some(focused_widget) = &mut self.focused_widget {
                        // The widgets are different.
                        if focused_widget.borrow().widget_id() != id_of_the_widget_that_gained_focus
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
                        *id_of_the_widget_that_gained_focus,
                    ));
                };
            }
        }

        Ok(widget_events)
    }

    ///
    pub fn next_widget_id(&mut self) -> WidgetId {
        self.next_widget_id_counter += 1;
        self.next_widget_id_counter
    }

    ///
    pub fn new_center(&mut self) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();

        // Add a new center layout widget.
        self.add_widget(Box::new(Center::new(
            widget_id,
            self.style.debug_rendering_stroke.clone(),
        )));

        widget_id
    }

    ///
    pub fn new_column(&mut self) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();

        // Add a new column layout widget.
        self.add_widget(Box::new(Column::new(
            widget_id,
            self.style.debug_rendering_stroke.clone(),
            HorizontalAlignment::Center,
            self.style.spacing,
        )));

        // Return the widget ID.
        widget_id
    }

    ///
    pub fn new_expanded(&mut self, flex_factor: u16) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();

        // Add a new expanded widget.
        self.add_widget(Box::new(Expanded::new(
            widget_id,
            self.style.debug_rendering_stroke.clone(),
            flex_factor,
        )));

        // Return the widget ID.
        widget_id
    }

    ///
    pub fn new_grid(
        &mut self,
        column_properties: GridColumnProperties,
        row_properties: GridRowProperties,
    ) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();

        // Add a new grid layout widget.
        self.add_widget(Box::new(Grid::new(
            widget_id,
            self.style.debug_rendering_stroke.clone(),
            column_properties,
            row_properties,
        )));

        // Return the widget ID.
        widget_id
    }

    ///
    pub fn new_hyper_link(&mut self, text: impl Into<String>) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();

        let mut font_unvisited = self.style.font.clone();
        font_unvisited.font_color = Color::rgb8(100, 100, 255);

        let mut font_being_clicked = self.style.font.clone();
        font_being_clicked.font_color = self.style.accent_color.clone();

        let mut font_visited = self.style.font.clone();
        font_visited.font_color = Color::rgb8(50, 50, 100);

        // Add a new hyperlink widget.
        self.add_widget(Box::new(Hyperlink::new(
            widget_id,
            self.style.debug_rendering_stroke.clone(),
            font_unvisited,
            font_being_clicked,
            font_visited,
            text,
        )));

        // Return the widget ID.
        widget_id
    }

    ///
    pub fn new_padding(&mut self) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();

        // Add a new padding widget.
        self.add_widget(Box::new(Padding::new(
            widget_id,
            self.style.debug_rendering_stroke.clone(),
            self.style.padding,
            self.style.padding,
            self.style.padding,
            self.style.padding,
        )));

        // Return the widget ID.
        widget_id
    }

    ///
    pub fn new_placeholder(&mut self, maximum_size: Size) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();

        // Add a new placeholder widget.
        self.add_widget(Box::new(Placeholder::new(
            widget_id,
            self.style.debug_rendering_stroke.clone(),
            maximum_size,
        )));

        // Return the widget ID.
        widget_id
    }

    ///
    pub fn new_row(&mut self) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();

        // Add a new row widget.
        self.add_widget(Box::new(Row::new(
            widget_id,
            self.style.debug_rendering_stroke.clone(),
            self.style.vertical_alignment,
            self.style.spacing,
        )));

        // Return the widget ID.
        widget_id
    }

    ///
    pub fn new_sized_box(&mut self, desired_size: Size) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();

        // Add a new sized box layout widget.
        self.add_widget(Box::new(SizedBox::new(
            widget_id,
            self.style.debug_rendering_stroke.clone(),
            desired_size,
        )));

        // Return the widget ID.
        widget_id
    }

    ///
    pub fn new_text(&mut self, text: impl Into<String>) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();

        // Add a new text widget.
        self.add_widget(Box::new(Text::new(
            widget_id,
            self.style.debug_rendering_stroke.clone(),
            self.style.font.clone(),
            text,
        )));

        // Return the widget ID.
        widget_id
    }

    ///
    pub fn new_text_button(&mut self, text: impl Into<String>) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();
        let child_widget_id = self.next_widget_id();

        // Add a new button with a text as inner child widget.
        self.add_widget(Box::new(Button::new(
            widget_id,
            self.style.debug_rendering_stroke.clone(),
            Rc::new(RefCell::new(Box::new(Text::new(
                child_widget_id,
                self.style.debug_rendering_stroke.clone(),
                self.style.font.clone(),
                text,
            )))),
            Some(PaintBrush::Color(self.style.accent_color.clone())),
            Some(self.style.frame_color.clone()),
            Some(self.style.accent_color.clone()),
        )));

        // Return the widget ID.
        widget_id
    }

    ///
    pub fn new_text_input(&mut self, text: impl Into<String>, width: f64) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();

        // Add a new text input widget.
        self.add_widget(Box::new(TextInput::new(
            widget_id,
            self.style.debug_rendering_stroke.clone(),
            self.style.font.clone(),
            text,
            width,
            self.style.frame_color.clone(),
            self.style.accent_color.clone(),
        )));

        // Return the widget ID.
        widget_id
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
            // Remove the parent widget ID for the current child widget ID.
            self.parent_widget_id_per_widget_id.remove(&child_widget_id);
        }
    }

    ///
    pub fn resize(&mut self, size: Size) {
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

    ///
    pub fn send_command(&mut self, command: Command) -> Result<(), WidgetError> {
        self.send_commands(vec![command])
    }

    ///
    pub fn send_commands(&mut self, commands: Vec<Command>) -> Result<(), WidgetError> {
        // Iterate over the given commands.
        for command in commands {
            // Get the ID of the widget from the command.
            let widget_id = *command.widget_id();

            // There is a widget with the given ID.
            let widget_box = if let Some(widget_box) = self.widgets.get(&widget_id) {
                widget_box
            }
            // There is no widget with the given ID.
            else {
                return Err(WidgetError::NoSuchWidget(widget_id));
            };

            match command {
                Command::AddChild {
                    widget_placement,
                    child_widget_id,
                    ..
                } => {
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
                Command::Destroy(_widget_id) => self.destroy_widget(widget_id),
                Command::RemoveChild {
                    parent_widget_id,
                    child_widget_id,
                    destroy_child_widget: destroy,
                } => {
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
                Command::SetChild {
                    parent_widget_id: _,
                    column_index: column,
                    row_index: row,
                    child_widget_id,
                } => {
                    // There is a widget with the given child widget ID.
                    let child_widget_box =
                        if let Some(widget_box) = self.widgets.get(&child_widget_id) {
                            widget_box
                        }
                        // There is no widget with the given ID.
                        else {
                            return Err(WidgetError::NoSuchWidget(child_widget_id));
                        };

                    widget_box
                        .borrow_mut()
                        .set_child(column, row, child_widget_box.clone())?;

                    self.add_parent_child_widget_connection(widget_id, child_widget_id);
                }
                Command::SetDebugRendering(_widget_id, debug_rendering) => {
                    widget_box.borrow_mut().set_debug_rendering(debug_rendering);
                }
                Command::SetFill(_widget_id, fill) => {
                    widget_box.borrow_mut().set_fill(fill)?;
                }
                Command::SetFont(_widget_id, font) => {
                    widget_box.borrow_mut().set_font(font)?;
                }
                Command::SetHasFocus(_widget_id, has_focus) => {
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
                        // Remember the current widget as focused.
                        self.focused_widget = Some(widget_box.clone());

                        // Tell the widget it has focus now.
                        widget_box.borrow_mut().set_has_focus(has_focus)?;
                    }
                }
                Command::SetHorizontalAlignment(_widget_id, horizontal_alignment) => {
                    widget_box
                        .borrow_mut()
                        .set_horizontal_alignment(horizontal_alignment)?;
                }
                Command::SetIsDisabled(_widget_id, is_disabled) => {
                    widget_box.borrow_mut().set_is_disabled(is_disabled);
                }
                Command::SetIsHidden(_widget_id, is_hidden) => {
                    widget_box.borrow_mut().set_is_hidden(is_hidden);
                }
                Command::SetMainWidget(_widget_id) => {
                    widget_box.borrow_mut().set_origin((1.0, 1.0).into());
                    self.main_widget = Some(widget_box.clone());
                }
                Command::SetStroke(_widget_id, stroke) => {
                    widget_box.borrow_mut().set_stroke(stroke)?;
                }
                Command::SetValue(_widget_id, value) => {
                    widget_box.borrow_mut().set_value(value)?;
                }
                Command::SetVerticalAlignment(_widget_id, vertical_alignment) => {
                    widget_box
                        .borrow_mut()
                        .set_vertical_alignment(vertical_alignment)?;
                }
            };
        }

        // There is a main widget.
        if let Some(main_widget) = &mut self.main_widget {
            // The widget commands might have affected the layout.
            // Apply the size constraints again for re-layout.
            main_widget
                .borrow_mut()
                .apply_size_constraints(self.size_constraints);
        }

        Ok(())
    }
}

impl Default for WidgetManager {
    fn default() -> Self {
        Self::new()
    }
}
