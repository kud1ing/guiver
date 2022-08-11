use guiver::{run, Application, Command, Event, Piet, Region, Size, WidgetManager};

pub(crate) struct App {
    widget_manager: WidgetManager,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = WidgetManager::new();

        // Create the widgets.
        let padding = widget_manager.new_padding();
        let text = widget_manager.new_text("The placeholders are expanded");
        let column = widget_manager.new_column();
        let expanded1 = widget_manager.new_expanded(1);
        let expanded2 = widget_manager.new_expanded(1);
        let placeholder1 = widget_manager.new_placeholder(Size::new(100.0, 50.0));
        let placeholder2 = widget_manager.new_placeholder(Size::new(100.0, 50.0));

        // Compose the widgets.
        widget_manager
            .send_commands(vec![
                Command::SetMainWidget(padding),
                Command::AppendChild(padding, column),
                Command::AppendChild(column, expanded1),
                Command::AppendChild(column, text),
                Command::AppendChild(column, expanded2),
                Command::AppendChild(expanded1, placeholder1),
                Command::AppendChild(expanded2, placeholder2),
            ])
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
}

pub fn main() {
    run(Box::new(App::new()), "expanded column", (300.0, 150.0).into());
}
