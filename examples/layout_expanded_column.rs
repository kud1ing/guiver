use guiver::{run, Application, Clipboard, Command, Event, Piet, Region, Size, WidgetManager};

pub(crate) struct App {
    widget_manager: WidgetManager<()>,
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
                Command::AddChild {
                    parent_widget_id: padding,
                    widget_placement: None,
                    child_widget_id: column,
                },
                Command::AddChild {
                    parent_widget_id: column,
                    widget_placement: None,
                    child_widget_id: expanded1,
                },
                Command::AddChild {
                    parent_widget_id: column,
                    widget_placement: None,
                    child_widget_id: text,
                },
                Command::AddChild {
                    parent_widget_id: column,
                    widget_placement: None,
                    child_widget_id: expanded2,
                },
                Command::AddChild {
                    parent_widget_id: expanded1,
                    widget_placement: None,
                    child_widget_id: placeholder1,
                },
                Command::AddChild {
                    parent_widget_id: expanded2,
                    widget_placement: None,
                    child_widget_id: placeholder2,
                },
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
    fn set_clipboard(&mut self, _clipboard: Clipboard) {}
}

pub fn main() {
    run(
        Box::new(App::new()),
        "expanded column",
        (300.0, 150.0).into(),
    );
}
