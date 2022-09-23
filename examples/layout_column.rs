use guiver::{
    run, Application, Clipboard, Command, Event, HorizontalAlignment, Piet, Region, Size,
    VerticalAlignment, WidgetManager,
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
        let placeholder1 = widget_manager.new_placeholder(Size::new(200.0, 50.0));
        let text3 = widget_manager.new_text("This is a right aligned text");
        let placeholder2 = widget_manager.new_placeholder(Size::new(100.0, 50.0));
        let placeholder3 = widget_manager.new_placeholder(Size::new(100.0, 50.0));

        // Compose the widgets.
        widget_manager
            .send_commands(vec![
                Command::SetMainWidget(padding),
                Command::AddChild(padding, row),
                Command::AddChild(row, column1),
                Command::AddChild(column1, text1),
                Command::AddChild(column1, text2),
                Command::AddChild(column1, placeholder3),
                Command::AddChild(row, column2),
                Command::AddChild(column2, placeholder1),
                Command::AddChild(column2, text3),
                Command::AddChild(row, placeholder2),
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
        let _widget_events = self.widget_manager.handle_event(event, None);
    }

    fn paint(&mut self, piet: &mut Piet, region: &Region) {
        self.widget_manager.paint(piet, region).unwrap();
    }

    fn resize(&mut self, size: Size) {
        self.widget_manager.resize(size);
    }
    fn set_clipboard(&mut self, _clipboard: Clipboard) {}
}

pub fn main() {
    run(Box::new(App::new()), "column", (400.0, 200.0).into());
}
