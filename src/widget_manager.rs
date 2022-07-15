use crate::widget::{Label, WidgetCommand};
use crate::{SizeConstraints, UserEvent, Widget, WidgetEvent, WidgetId};
use druid_shell::kurbo::Size;
use druid_shell::piet::Piet;
use druid_shell::Region;
use std::collections::HashMap;

///
pub struct WidgetManager {
    /// Widgets that were added with `add_widget()` but are not part of a parent widget yet.
    added_widgets: HashMap<WidgetId, Box<dyn Widget>>,
    /// The main widget that fills the whole window.
    main_widget: Box<dyn Widget>,
    /// The counter for the next widget ID.
    next_widget_id_counter: WidgetId,
    /// The size constraints so that the main widget fills the whole window. It is set by the
    /// window event handler for each window resize.
    size_constraints: SizeConstraints,
}

impl WidgetManager {
    ///
    pub fn new(mut main_widget: Box<dyn Widget>) -> Self {
        // Set the main widget's origin.
        main_widget.set_origin((0.0, 0.0).into());

        WidgetManager {
            added_widgets: HashMap::new(),
            main_widget,
            next_widget_id_counter: 0,
            size_constraints: SizeConstraints::default(),
        }
    }

    ///
    pub fn add_widget(&mut self, widget_id: WidgetId, widget: Box<dyn Widget>) {
        self.added_widgets.insert(widget_id, widget);
    }

    ///
    pub fn handle_event(&mut self, event: &UserEvent) -> Vec<WidgetEvent> {
        let mut widget_events = vec![];

        // Let the main widget handle the given user event.
        self.main_widget.handle_event(event, &mut widget_events);

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
        // Paint the main widget.
        self.main_widget.paint(piet, region)
    }

    ///
    pub fn resize(&mut self, size: Size) {
        // Create and use a new size constraint.
        let size_constraints = SizeConstraints::tight(size);
        self.size_constraints = size_constraints;

        // Resize the main widget.
        self.main_widget.apply_size_constraints(size_constraints);
    }

    ///
    pub fn send_command(&mut self, widget_id: WidgetId, widget_command: WidgetCommand) {
        // TODO: call `send_command_dictionary()` instead
        self.send_commands(vec![(widget_id, widget_command)]);
    }

    ///
    pub fn send_command_dictionary(
        &mut self,
        widget_commands: &HashMap<WidgetId, Vec<WidgetCommand>>,
    ) {
        // Let the main widget handle the given widget commands.
        self.main_widget.handle_commands(widget_commands);

        // The widget command might have affected the layout.
        // Resize the main widget.
        self.main_widget
            .apply_size_constraints(self.size_constraints);
    }

    ///
    pub fn send_commands(&mut self, widget_commands: Vec<(WidgetId, WidgetCommand)>) {
        let mut widget_command_dictionary = HashMap::new();

        // Collect the widget commands in a dictionary.
        for (widget_id, widget_command) in widget_commands {
            widget_command_dictionary
                .entry(widget_id)
                .or_insert(vec![])
                .push(widget_command);
        }

        self.send_command_dictionary(&widget_command_dictionary);
    }

    ///
    pub fn set_main_widget(&mut self, mut main_widget: Box<dyn Widget>) {
        // Set the main widget's origin.
        main_widget.set_origin((0.0, 0.0).into());

        self.main_widget = main_widget;

        // Resize the main widget.
        self.main_widget
            .apply_size_constraints(self.size_constraints);
    }
}
