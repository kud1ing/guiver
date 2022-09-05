use guiver::{run, Application, Clipboard, Command, Event, Piet, Region, Size, WidgetManager};

pub(crate) struct App {
    widget_manager: WidgetManager,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = WidgetManager::new();

        // Create the widget.
        let placeholder = widget_manager.new_placeholder(Size::new(100.0, 50.0));

        // Compose the widget.
        widget_manager
            .send_commands(vec![Command::SetMainWidget(placeholder)])
            .unwrap();

        App { widget_manager }
    }
}

impl Application for App {
    fn handle_event(&mut self, _system_event_event: &Event) {}

    fn paint(&mut self, piet: &mut Piet, region: &Region) {
        self.widget_manager.paint(piet, region).unwrap();
    }

    fn resize(&mut self, size: Size) {
        self.widget_manager.resize(size);
    }

    fn set_clipboard(&mut self, _clipboard: Clipboard) {}
}

pub fn main() {
    run(Box::new(App::new()), "placeholder", (300.0, 150.0).into());
}
