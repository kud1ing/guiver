/**
This implements the "Counter" task from [7GUIs](https://eugenkiss.github.io/7guis/tasks/).
*/
use guiver::{run, Application, Command, Piet, Region, Size, SystemEvent, WidgetManager};

pub(crate) struct App {
    widget_manager: WidgetManager,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = WidgetManager::new();

        // Create the widget.
        let placeholder = widget_manager.new_placeholder();

        // Compose the widget.
        widget_manager
            .send_commands(vec![Command::SetMainWidget(placeholder)])
            .unwrap();

        App { widget_manager }
    }
}

impl Application for App {
    fn handle_system_event(&mut self, _system_event: &SystemEvent) {}

    fn paint(&mut self, piet: &mut Piet, region: &Region) {
        // Paint the main widget.
        self.widget_manager.paint(piet, region).unwrap();
    }

    fn resize(&mut self, size: Size) {
        // Resize the main widget.
        self.widget_manager.resize(size);
    }
}

pub fn main() {
    run(Box::new(App::new()), "placeholder", (300.0, 150.0).into());
}
