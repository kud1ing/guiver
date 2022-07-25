use crate::style::Style;
use crate::widget::layout::{Column, Padding, Row};
use crate::widget::{Button, Placeholder, Text, TextInput, WidgetCommand, WidgetError};
use crate::{SizeConstraints, SystemEvent, Widget, WidgetEvent, WidgetId};
use druid_shell::kurbo::Size;
use druid_shell::piet::Piet;
use druid_shell::{piet, Region};
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

///
pub type WidgetBox = Rc<RefCell<Box<dyn Widget>>>;

/// A command to the widget manager or widgets.
#[derive(Debug)]
pub enum Command {
    /// Append the child widget.
    AppendChild(WidgetId, WidgetId),
    /// Remove the widget's children.
    RemoveAllChildren(WidgetId),
    /// Remove the child widget.
    RemoveChild(WidgetId, WidgetId),
    /// Enables/disables debug rendering mode for the widget.
    SetDebugRendering(WidgetId, bool),
    /// Gives/removes focus to the widget.
    SetHasFocus(WidgetId, bool),
    /// Enables/disables the widget.
    SetIsDisabled(WidgetId, bool),
    /// Hides/shows the widget.
    SetIsHidden(WidgetId, bool),
    /// Makes the widget with the given ID the main widget.
    SetMainWidget(WidgetId),
    /// Sets the given value to the widget.
    SetValue(WidgetId, Box<dyn Any>),
}

impl Command {
    /// Returns the ID of the receiver widget.
    pub fn widget_id(&self) -> &WidgetId {
        return match self {
            Command::AppendChild(widget_id, _) => &widget_id,
            Command::RemoveAllChildren(widget_id) => &widget_id,
            Command::RemoveChild(widget_id, _) => &widget_id,
            Command::SetDebugRendering(widget_id, _) => &widget_id,
            Command::SetHasFocus(widget_id, _) => &widget_id,
            Command::SetIsDisabled(widget_id, _) => &widget_id,
            Command::SetIsHidden(widget_id, _) => &widget_id,
            Command::SetMainWidget(widget_id) => &widget_id,
            Command::SetValue(widget_id, _) => &widget_id,
        };
    }
}

// =================================================================================================

///
pub struct WidgetManager {
    /// The widget that has the focus.
    focused_widget: Option<WidgetBox>,
    /// The main widget that fills the whole window.
    main_widget: Option<WidgetBox>,
    /// The counter for the next widget ID.
    next_widget_id_counter: WidgetId,
    /// The size constraints. It is set in `resize()`, called by the window event handler for every
    /// window resize event so that the main widget fills the whole window.
    size_constraints: SizeConstraints,
    ///
    style: Style,
    /// All widgets per widget ID. This is used:
    /// * to pass commands to a widget
    widgets: HashMap<WidgetId, WidgetBox>,
}

impl WidgetManager {
    ///
    pub fn new() -> Self {
        WidgetManager {
            focused_widget: None,
            main_widget: None,
            next_widget_id_counter: 0,
            size_constraints: SizeConstraints::default(),
            style: Style::default(),
            widgets: HashMap::new(),
        }
    }

    ///
    pub fn add_widget(&mut self, widget_id: WidgetId, widget: Box<dyn Widget>) {
        self.widgets
            .insert(widget_id, Rc::new(RefCell::new(widget)));
    }

    ///
    pub fn collect_garbage(&mut self) {
        // TODO
        println!("TODO: `collect_garbage()`");
    }

    ///
    pub fn handle_event(
        &mut self,
        system_event: &SystemEvent,
    ) -> Result<Vec<WidgetEvent>, WidgetError> {
        let mut widget_events = vec![];

        // A widget has focus.
        if let Some(focused_widget) = &mut self.focused_widget {
            match system_event {
                SystemEvent::KeyDown(_) => {
                    // Let only the focused widget handle the key event.
                    focused_widget
                        .borrow_mut()
                        .handle_event(system_event, &mut widget_events);
                    return Ok(widget_events);
                }
                SystemEvent::KeyUp(_) => {
                    // Let only the focused widget handle the key event.
                    focused_widget
                        .borrow_mut()
                        .handle_event(system_event, &mut widget_events);
                    return Ok(widget_events);
                }
                SystemEvent::MouseDown(_) => {}
                SystemEvent::MouseMove(_) => {}
                SystemEvent::MouseUp(_) => {}
            }
        }

        // There is a main widget.
        if let Some(main_widget) = &mut self.main_widget {
            // Let the main widget handle the given user event.
            main_widget
                .borrow_mut()
                .handle_event(system_event, &mut widget_events);
        }

        let mut id_of_the_last_widget_that_gained_focus = None;

        // Iterate over the widget event.
        for widget_event in &widget_events {
            match widget_event {
                WidgetEvent::Clicked(_) => {}
                WidgetEvent::GotFocus(widget_id) => {
                    // A widget gained focus.
                    id_of_the_last_widget_that_gained_focus = Some(widget_id);
                }
                WidgetEvent::LostFocus(widget_id) => {
                    // A widget had focus.
                    if let Some(focused_widget) = &mut self.focused_widget {
                        // The widgets was indeed focues.
                        if focused_widget.borrow().widget_id() != widget_id {
                            self.focused_widget = None;
                        }
                    }
                }
                WidgetEvent::ValueChanged(_, _) => {}
            }
        }

        // A widget gained focus.
        if let Some(id_of_the_widget_that_gained_focus) = id_of_the_last_widget_that_gained_focus {
            // There is a widget with the given ID.
            if let Some(widget_box) = self.widgets.get(id_of_the_widget_that_gained_focus) {
                // A widget had focus.
                if let Some(focused_widget) = &mut self.focused_widget {
                    // The widgets are different.
                    if focused_widget.borrow().widget_id() != id_of_the_widget_that_gained_focus {
                        // Unfocus that previously focused widget.
                        focused_widget
                            .borrow_mut()
                            .handle_command(WidgetCommand::SetHasFocus(false))?;
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

        Ok(widget_events)
    }

    ///
    fn next_widget_id(&mut self) -> WidgetId {
        self.next_widget_id_counter += 1;
        self.next_widget_id_counter
    }

    ///
    pub fn new_column(&mut self) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();

        // Add a new column widget.
        self.widgets.insert(
            widget_id,
            Rc::new(RefCell::new(Box::new(Column::new(
                widget_id,
                self.style.padding,
            )))),
        );

        // Return the column's widget ID.
        widget_id
    }

    ///
    pub fn new_padding(&mut self) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();

        // Add a new padding widget.
        self.widgets.insert(
            widget_id,
            Rc::new(RefCell::new(Box::new(Padding::new(
                widget_id,
                self.style.debug_rendering_stroke_brush.clone(),
                self.style.debug_rendering_stroke_width,
                self.style.padding,
                self.style.padding,
                self.style.padding,
                self.style.padding,
            )))),
        );

        // Return the padding's widget ID.
        widget_id
    }

    ///
    pub fn new_placeholder(&mut self) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();

        // Add a new placeholder widget.
        self.widgets.insert(
            widget_id,
            Rc::new(RefCell::new(Box::new(Placeholder::new(widget_id)))),
        );

        // Return the placeholder's widget ID.
        widget_id
    }

    ///
    pub fn new_row(&mut self) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();

        // Add a new row widget.
        self.widgets.insert(
            widget_id,
            Rc::new(RefCell::new(Box::new(Row::new(
                widget_id,
                self.style.debug_rendering_stroke_brush.clone(),
                self.style.debug_rendering_stroke_width,
                self.style.vertical_alignment,
                self.style.spacing,
            )))),
        );

        // Return the row's widget ID.
        widget_id
    }

    ///
    pub fn new_text(&mut self, text: impl Into<String>) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();

        // Add a new text widget.
        self.widgets.insert(
            widget_id,
            Rc::new(RefCell::new(Box::new(Text::new(
                widget_id,
                self.style.debug_rendering_stroke_brush.clone(),
                self.style.debug_rendering_stroke_width,
                text,
            )))),
        );

        // Return the text's widget ID.
        widget_id
    }

    ///
    pub fn new_text_button(&mut self, text: impl Into<String>) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();
        let child_widget_id = self.next_widget_id();

        // Add a new button with a text as inner child.
        self.widgets.insert(
            widget_id,
            Rc::new(RefCell::new(Box::new(Button::new(
                widget_id,
                Rc::new(RefCell::new(Box::new(Text::new(
                    child_widget_id,
                    self.style.debug_rendering_stroke_brush.clone(),
                    self.style.debug_rendering_stroke_width,
                    text,
                )))),
                Some(self.style.frame_color.clone()),
                Some(self.style.accent_color.clone()),
            )))),
        );

        // Return the button's widget ID.
        widget_id
    }

    ///
    pub fn new_text_input(&mut self, text: impl Into<String>, width: f64) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();

        // Add a new text input widget.
        self.widgets.insert(
            widget_id,
            Rc::new(RefCell::new(Box::new(TextInput::new(
                widget_id,
                self.style.debug_rendering_stroke_brush.clone(),
                self.style.debug_rendering_stroke_width,
                text,
                width,
                self.style.frame_color.clone(),
                self.style.accent_color.clone(),
            )))),
        );

        // Return the text input's widget ID.
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
            let widget_id = command.widget_id();

            // There is a widget with the given ID.
            let widget_box = if let Some(widget_box) = self.widgets.get(widget_id) {
                widget_box
            }
            // There is no widget with the given ID.
            else {
                return Err(WidgetError::NoSuchWidget(*widget_id));
            };

            match command {
                Command::AppendChild(_widget_id, child_id) => {
                    // There is a widget with the child ID from the command.
                    let child_widget_box =
                        if let Some(child_widget_box) = self.widgets.get(&child_id) {
                            child_widget_box
                        }
                        // There is no widget with the given child ID.
                        else {
                            return Err(WidgetError::NoSuchWidget(child_id));
                        };

                    widget_box
                        .borrow_mut()
                        .handle_command(WidgetCommand::AppendChild(child_widget_box.clone()))?;
                }
                Command::RemoveAllChildren(_widget_id) => {
                    widget_box
                        .borrow_mut()
                        .handle_command(WidgetCommand::RemoveAllChildren)?;
                }
                Command::RemoveChild(_widget_id, child_id) => {
                    widget_box
                        .borrow_mut()
                        .handle_command(WidgetCommand::RemoveChild(child_id))?;
                }
                Command::SetDebugRendering(_widget_id, debug_rendering) => {
                    widget_box
                        .borrow_mut()
                        .handle_command(WidgetCommand::SetDebugRendering(debug_rendering))?;
                }
                Command::SetHasFocus(_widget_id, has_focus) => {
                    // A widget had focus.
                    if let Some(focused_widget) = &mut self.focused_widget {
                        // TODO: only unfocus if the affected widgets are different.
                        // Unfocus that widget.
                        focused_widget
                            .borrow_mut()
                            .handle_command(WidgetCommand::SetHasFocus(false))?;
                    }

                    // Remember the current widget as focused.
                    self.focused_widget = Some(widget_box.clone());

                    // Tell the widget it has focus now.
                    widget_box
                        .borrow_mut()
                        .handle_command(WidgetCommand::SetHasFocus(has_focus))?;
                }
                Command::SetIsDisabled(_widget_id, is_disabled) => {
                    widget_box
                        .borrow_mut()
                        .handle_command(WidgetCommand::SetIsDisabled(is_disabled))?;
                }
                Command::SetIsHidden(_widget_id, is_hidden) => {
                    widget_box
                        .borrow_mut()
                        .handle_command(WidgetCommand::SetIsHidden(is_hidden))?;
                }
                Command::SetMainWidget(_widget_id) => {
                    widget_box.borrow_mut().set_origin((1.0, 1.0).into());
                    self.main_widget = Some(widget_box.clone());
                }
                Command::SetValue(_widget_id, value) => {
                    widget_box
                        .borrow_mut()
                        .handle_command(WidgetCommand::SetValue(value))?;
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
