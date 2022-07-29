// TODO: This implements the "Flight Booker" task from [7GUIs](https://eugenkiss.github.io/7guis/tasks/).
use guiver::{run, Application, Command, Event, Piet, Region, Size, WidgetManager};

pub(crate) struct App {
    widget_manager: WidgetManager,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = WidgetManager::new();

        // Create the widgets.
        let padding = widget_manager.new_padding();
        let column = widget_manager.new_column();
        let dropdown_box = widget_manager.new_placeholder();
        let text_input_start_date = widget_manager.new_text_input("", 100.0);
        let text_input_return_date = widget_manager.new_text_input("", 100.0);
        let book_button = widget_manager.new_text_button("Book");

        // Compose the widgets.
        widget_manager
            .send_commands(vec![
                Command::SetMainWidget(padding),
                Command::AppendChild(padding, column),
                Command::AppendChild(column, dropdown_box),
                Command::AppendChild(column, text_input_start_date),
                Command::AppendChild(column, text_input_return_date),
                Command::AppendChild(column, book_button),
            ])
            .unwrap();

        App { widget_manager }
    }
}

impl Application for App {
    fn handle_event(&mut self, event: &Event) {
        let _widget_events = self.widget_manager.handle_event(event).unwrap();
    }

    fn paint(&mut self, piet: &mut Piet, region: &Region) {
        self.widget_manager.paint(piet, region).unwrap();
    }

    fn resize(&mut self, size: Size) {
        self.widget_manager.resize(size);
    }
}

pub fn main() {
    run(
        Box::new(App::new()),
        "7GUIs: Flight Booker",
        (300.0, 150.0).into(),
    );
}