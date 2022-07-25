use guiver::{run, Application, Command, Piet, Region, Size, SystemEvent, WidgetManager};

pub(crate) struct App {
    widget_manager: WidgetManager,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = WidgetManager::new();

        // Create the widgets.
        let center = widget_manager.new_center();
        let text = widget_manager.new_text("This is a text at the center");

        // Compose the widgets.
        widget_manager
            .send_commands(vec![
                Command::SetMainWidget(center),
                Command::AppendChild(center, text),
            ])
            .unwrap();

        App { widget_manager }
    }
}

impl Application for App {
    fn handle_system_event(&mut self, _system_event: &SystemEvent) {}

    fn paint(&mut self, piet: &mut Piet, region: &Region) {
        self.widget_manager.paint(piet, region).unwrap();
    }

    fn resize(&mut self, size: Size) {
        self.widget_manager.resize(size);
    }
}

pub fn main() {
    run(Box::new(App::new()), "center", (400.0, 200.0).into());
}