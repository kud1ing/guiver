use guiver::{
    run, Application, Command, Event, HorizontalAlignment, Piet, Region, Size, VerticalAlignment,
    WidgetManager,
};

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
        let text1 = widget_manager.new_text("This is a text");
        let text2 = widget_manager.new_text_button("This is a button");
        let placeholder1 = widget_manager.new_placeholder();
        let text3 = widget_manager.new_text("This is a right aligned text");
        let placeholder2 = widget_manager.new_placeholder();
        let placeholder3 = widget_manager.new_placeholder();

        // Compose the widgets.
        widget_manager
            .send_commands(vec![
                Command::SetMainWidget(padding),
                Command::AppendChild(padding, row),
                Command::AppendChild(row, column1),
                Command::AppendChild(column1, text1),
                Command::AppendChild(column1, text2),
                Command::AppendChild(column1, placeholder3),
                Command::AppendChild(row, column2),
                Command::AppendChild(column2, placeholder1),
                Command::AppendChild(column2, text3),
                Command::AppendChild(row, placeholder2),
                //
                Command::SetVerticalAlignment(row, VerticalAlignment::Top),
                Command::SetHorizontalAlignment(column1, HorizontalAlignment::Left),
                Command::SetHorizontalAlignment(column2, HorizontalAlignment::Right),
            ])
            .unwrap();

        App { widget_manager }
    }
}

impl Application for App {
    fn handle_event(&mut self, event: &Event) {
        let _widget_events = self.widget_manager.handle_event(event);
    }

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
