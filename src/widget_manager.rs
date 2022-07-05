use crate::widget::{Label, WidgetCommand};
use crate::{SizeConstraints, UserEvent, Widget, WidgetEvent, WidgetId};
use druid_shell::kurbo::Size;
use druid_shell::piet::Piet;
use druid_shell::Region;
use std::collections::HashMap;

///(
pub struct WidgetManager {
    added_widgets: HashMap<WidgetId, Box<dyn Widget>>,
    main_widget: Box<dyn Widget>,
    next_widget_id: WidgetId,
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
            next_widget_id: 0,
            size_constraints: SizeConstraints::default(),
        }
    }

    ///
    pub fn new_label(&mut self, text: impl Into<String>) -> WidgetId {
        let widget_id = self.next_widget_id();

        self.added_widgets
            .insert(widget_id, Box::new(Label::new(widget_id, text)));

        widget_id
    }

    ///
    pub fn handle_user_event(&mut self, event: &UserEvent) -> Vec<WidgetEvent> {
        let mut widget_events = vec![];

        // Let the main widget handle the given user event.
        self.main_widget
            .handle_user_event(event, &mut widget_events);

        widget_events
    }

    ///
    pub fn handle_widget_command(&mut self, widget_command: &WidgetCommand) {
        self.main_widget.handle_widget_command(widget_command);

        // The widget command might have affected the layout.
        // Resize the main widget.
        self.main_widget
            .apply_size_constraints(self.size_constraints);
    }

    ///
    fn next_widget_id(&mut self) -> WidgetId {
        self.next_widget_id += 1;
        self.next_widget_id
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
    pub fn set_main_widget(&mut self, mut main_widget: Box<dyn Widget>) {
        // Set the main widget's origin.
        main_widget.set_origin((0.0, 0.0).into());

        self.main_widget = main_widget;

        // Resize the main widget.
        self.main_widget
            .apply_size_constraints(self.size_constraints);
    }
}
