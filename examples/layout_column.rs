use guiver::{run, Application,Region, Command, Piet, Size, SystemEvent, WidgetManager};

pub(crate) struct App {
    widget_manager: WidgetManager,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = WidgetManager::new();

        // Create the widgets.
        let padding = widget_manager.new_padding();
        let row = widget_manager.new_row();
        let column1 = widget_manager.new_column();
        let column2 = widget_manager.new_column();
        let placeholder1 = widget_manager.new_placeholder();
        let placeholder2 = widget_manager.new_placeholder();
        let placeholder3 = widget_manager.new_placeholder();
        let placeholder4 = widget_manager.new_placeholder();
        let placeholder5 = widget_manager.new_placeholder();
        let placeholder6 = widget_manager.new_placeholder();

        // Compose the widgets.
        widget_manager
            .send_commands(vec![
                Command::SetMainWidget(padding),
                Command::AppendChild(padding, row),
                Command::AppendChild(row, column1),
                Command::AppendChild(column1, placeholder1),
                Command::AppendChild(column1, placeholder2),
                Command::AppendChild(column1, placeholder3),
                Command::AppendChild(row, column2),
                Command::AppendChild(column2, placeholder4),
                Command::AppendChild(column2, placeholder5),
                Command::AppendChild(row, placeholder6),
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
    run(Box::new(App::new()), "column", (400.0, 200.0).into());
}
