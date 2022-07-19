use crate::widget::{Label, WidgetCommand, WidgetError};
use crate::{SizeConstraints, SystemEvent, Widget, WidgetEvent, WidgetId};
use druid_shell::kurbo::Size;
use druid_shell::piet::Piet;
use druid_shell::Region;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// A command for the widget manager.
#[derive(Debug)]
pub enum WidgetManagerCommand {
    /// Remove the widget's children.
    Clear(WidgetId),
    /// Remove the widget.
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

///
pub struct WidgetManager {
    /// The main widget that fills the whole window.
    main_widget: Option<Box<dyn Widget>>,
    /// The counter for the next widget ID.
    next_widget_id_counter: WidgetId,
    /// The size constraints so that the main widget fills the whole window. It is set by the
    /// window event handler for each window resize event.
    size_constraints: SizeConstraints,
    /// All widgets per widget ID. This is used:
    /// * to pass messages to widgets
    widgets: HashMap<WidgetId, Rc<RefCell<Box<dyn Widget>>>>,
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
    pub fn handle_event(&mut self, system_event: &SystemEvent) -> Vec<WidgetEvent> {
        let mut widget_events = vec![];

        // There is a main widget.
        if let Some(main_widget) = &mut self.main_widget {
            // Let the main widget handle the given user event.
            main_widget.handle_event(system_event, &mut widget_events);
        }

        widget_events
    }

    ///
    pub fn new_label(&mut self, text: impl Into<String>) -> WidgetId {
        let widget_id = self.next_widget_id();

        self.add_widget(widget_id, Box::new(Label::new(widget_id, text)));

        widget_id
    }

    ///
    fn next_widget_id(&mut self) -> WidgetId {
        self.next_widget_id_counter += 1;
        self.next_widget_id_counter
    }

    ///
    pub fn paint(&self, piet: &mut Piet, region: &Region) {
        // There is a main widget.
        if let Some(main_widget) = &self.main_widget {
            // Paint the main widget.
            main_widget.paint(piet, region)
        }
    }

    ///
    pub fn resize(&mut self, size: Size) {
        // Create a new size constraint.
        let size_constraints = SizeConstraints::tight(size);

        // Use the new size constraint.
        self.size_constraints = size_constraints;

        // There is a main widget.
        if let Some(main_widget) = &mut self.main_widget {
            // Resize the main widget.
            main_widget.apply_size_constraints(size_constraints);
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
        let mut widget_command_dictionary: HashMap<WidgetId, Vec<WidgetCommand>> = HashMap::new();

        // Collect the commands in a dictionary.
        for command in commands {
            match command {
                WidgetManagerCommand::Clear(widget_id) => {
                    widget_command_dictionary
                        .entry(widget_id)
                        .or_insert(vec![])
                        .push(WidgetCommand::Clear);
                }
                WidgetManagerCommand::RemoveChild(parent_id, child_id) => {
                    widget_command_dictionary
                        .entry(parent_id)
                        .or_insert(vec![])
                        .push(WidgetCommand::RemoveChild(child_id));
                }
                WidgetManagerCommand::SetHasFocus(widget_id, has_focus) => {
                    widget_command_dictionary
                        .entry(widget_id)
                        .or_insert(vec![])
                        .push(WidgetCommand::SetHasFocus(has_focus));
                }
                WidgetManagerCommand::SetIsDisabled(widget_id, is_disabled) => {
                    widget_command_dictionary
                        .entry(widget_id)
                        .or_insert(vec![])
                        .push(WidgetCommand::SetIsDisabled(is_disabled));
                }
                WidgetManagerCommand::SetIsHidden(widget_id, is_hidden) => {
                    widget_command_dictionary
                        .entry(widget_id)
                        .or_insert(vec![])
                        .push(WidgetCommand::SetIsHidden(is_hidden));
                }
                WidgetManagerCommand::SetMainWidget(_widget_id) => {
                    // TODO
                    println!("`WidgetManager::send_commands(SetMainWidget)`: TODO");

                    /*
                    // Set the main widget's origin.
                    main_widget.set_origin((0.0, 0.0).into());

                    // Resize the main widget.
                    main_widget.apply_size_constraints(self.size_constraints);

                    self.main_widget = Some(main_widget);
                    */
                }
                WidgetManagerCommand::SetValue(widget_id, value) => {
                    widget_command_dictionary
                        .entry(widget_id)
                        .or_insert(vec![])
                        .push(WidgetCommand::SetValue(value));
                }
            };
        }

        // There is a main widget.
        if let Some(main_widget) = &mut self.main_widget {
            // Let the main widget handle the given widget commands.
            main_widget.handle_commands(&widget_command_dictionary)?;

            // The widget command might have affected the layout.
            // Resize the main widget.
            main_widget.apply_size_constraints(self.size_constraints);
        }

        Ok(())
    }
}
