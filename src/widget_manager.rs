use crate::widget::layout::{Padding, Row};
use crate::widget::{Label, Placeholder, WidgetCommand, WidgetError};
use crate::{SizeConstraints, SystemEvent, Widget, WidgetEvent, WidgetId};
use druid_shell::kurbo::Size;
use druid_shell::piet::Piet;
use druid_shell::Region;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

///
pub type WidgetBox = Rc<RefCell<Box<dyn Widget>>>;

/// A command for the widget manager.
#[derive(Debug)]
pub enum WidgetManagerCommand {
    /// Append the child widget.
    AppendChild(WidgetId, WidgetId),
    /// Remove the widget's children.
    Clear(WidgetId),
    /// Remove the child widget.
    RemoveChild(WidgetId, WidgetId),
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

impl WidgetManagerCommand {
    /// Returns the ID of the receiver widget.
    pub fn widget_id(&self) -> &WidgetId {
        return match self {
            WidgetManagerCommand::AppendChild(widget_id, _) => &widget_id,
            WidgetManagerCommand::Clear(widget_id) => &widget_id,
            WidgetManagerCommand::RemoveChild(widget_id, _) => &widget_id,
            WidgetManagerCommand::SetHasFocus(widget_id, _) => &widget_id,
            WidgetManagerCommand::SetIsDisabled(widget_id, _) => &widget_id,
            WidgetManagerCommand::SetIsHidden(widget_id, _) => &widget_id,
            WidgetManagerCommand::SetMainWidget(widget_id) => &widget_id,
            WidgetManagerCommand::SetValue(widget_id, _) => &widget_id,
        };
    }
}

// =================================================================================================

///
pub struct WidgetManager {
    /// The main widget that fills the whole window.
    main_widget: Option<WidgetBox>,
    /// The counter for the next widget ID.
    next_widget_id_counter: WidgetId,
    /// The size constraints. It is set in `resize()`, called by the window event handler for every
    /// window resize event so that the main widget fills the whole window.
    size_constraints: SizeConstraints,
    /// All widgets per widget ID. This is used:
    /// * to pass commands to a widget
    widgets: HashMap<WidgetId, WidgetBox>,
}

impl WidgetManager {
    ///
    pub fn new() -> Self {
        WidgetManager {
            main_widget: None,
            next_widget_id_counter: 0,
            size_constraints: SizeConstraints::default(),
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
    pub fn handle_event(&mut self, system_event: &SystemEvent) -> Vec<WidgetEvent> {
        let mut widget_events = vec![];

        // There is a main widget.
        if let Some(main_widget) = &mut self.main_widget {
            // Let the main widget handle the given user event.
            main_widget
                .borrow_mut()
                .handle_event(system_event, &mut widget_events);
        }

        widget_events
    }

    ///
    fn next_widget_id(&mut self) -> WidgetId {
        self.next_widget_id_counter += 1;
        self.next_widget_id_counter
    }

    ///
    pub fn new_label(&mut self, text: impl Into<String>) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();

        // Add a new label widget.
        self.widgets.insert(
            widget_id,
            Rc::new(RefCell::new(Box::new(Label::new(widget_id, text)))),
        );

        // Return the label's widget ID.
        widget_id
    }

    ///
    pub fn new_padding(
        &mut self,
        padding_left: f64,
        padding_top: f64,
        padding_right: f64,
        padding_bottom: f64,
    ) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();

        // Add a new padding widget.
        self.widgets.insert(
            widget_id,
            Rc::new(RefCell::new(Box::new(Padding::new(
                widget_id,
                padding_left,
                padding_top,
                padding_right,
                padding_bottom,
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
    pub fn new_row(&mut self, spacing: f64) -> WidgetId {
        // Get a new widget ID.
        let widget_id = self.next_widget_id();

        // Add a new row widget.
        self.widgets.insert(
            widget_id,
            Rc::new(RefCell::new(Box::new(Row::new(widget_id, spacing)))),
        );

        // Return the row's widget ID.
        widget_id
    }

    ///
    pub fn paint(&self, piet: &mut Piet, region: &Region) {
        // There is a main widget.
        if let Some(main_widget) = &self.main_widget {
            // Paint the main widget.
            main_widget.borrow().paint(piet, region)
        }
    }

    ///
    pub fn resize(&mut self, size: Size) {
        // Create a new size constraint from the given window size.
        let size_constraints = SizeConstraints::tight(size);

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
    pub fn send_command(&mut self, command: WidgetManagerCommand) -> Result<(), WidgetError> {
        self.send_commands(vec![command])
    }

    ///
    pub fn send_commands(
        &mut self,
        commands: Vec<WidgetManagerCommand>,
    ) -> Result<(), WidgetError> {
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
                WidgetManagerCommand::AppendChild(_widget_id, child_id) => {
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
                WidgetManagerCommand::Clear(_widget_id) => {
                    widget_box
                        .borrow_mut()
                        .handle_command(WidgetCommand::Clear)?;
                }
                WidgetManagerCommand::RemoveChild(_widget_id, child_id) => {
                    widget_box
                        .borrow_mut()
                        .handle_command(WidgetCommand::RemoveChild(child_id))?;
                }
                WidgetManagerCommand::SetHasFocus(_widget_id, has_focus) => {
                    widget_box
                        .borrow_mut()
                        .handle_command(WidgetCommand::SetHasFocus(has_focus))?;
                }
                WidgetManagerCommand::SetIsDisabled(_widget_id, is_disabled) => {
                    widget_box
                        .borrow_mut()
                        .handle_command(WidgetCommand::SetIsDisabled(is_disabled))?;
                }
                WidgetManagerCommand::SetIsHidden(_widget_id, is_hidden) => {
                    widget_box
                        .borrow_mut()
                        .handle_command(WidgetCommand::SetIsHidden(is_hidden))?;
                }
                WidgetManagerCommand::SetMainWidget(_widget_id) => {
                    self.main_widget = Some(widget_box.clone());
                }
                WidgetManagerCommand::SetValue(_widget_id, value) => {
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
