/**
This implements the "Counter" task from [7GUIs](https://eugenkiss.github.io/7guis/tasks/).
*/
use druid_shell::kurbo::Size;
use druid_shell::piet::Piet;
use druid_shell::Region;
use guiver::widget::layout::{Padding, Row};
use guiver::widget::Label;
use guiver::{run, WidgetEvent, WidgetId, WidgetManager, WidgetManagerCommand};
use guiver::{Application, SystemEvent};

pub(crate) struct App {
    widget_manager: WidgetManager,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = WidgetManager::new();

        // Create the widget.
        let placeholder = widget_manager.new_placeholder();

        // Compose the widget.
        let _ =
            widget_manager.send_commands(vec![WidgetManagerCommand::SetMainWidget(placeholder)]);

        App { widget_manager }
    }
}

impl Application for App {
    fn handle_system_event(&mut self, system_event: &SystemEvent) {
        // Do nothing.
    }

    fn paint(&mut self, piet: &mut Piet, region: &Region) {
        // Paint the main widget.
        self.widget_manager.paint(piet, region);
    }

    fn resize(&mut self, size: Size) {
        // Resize the main widget.
        self.widget_manager.resize(size);
    }
}

pub fn main() {
    run(Box::new(App::new()), "placeholder", (300.0, 150.0).into());
}
