use guiver::{Command, Size, WidgetManager, WidgetType};
use guiver_piet::{run, Clipboard, Event, Piet, PietApplication, PietWidgetManager, Region};

pub(crate) struct App {
    widget_manager: PietWidgetManager<()>,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = PietWidgetManager::new();

        let center = widget_manager.widget_id_provider().next_widget_id();
        let text = widget_manager.widget_id_provider().next_widget_id();

        // Compose the widgets.
        widget_manager
            .handle_commands(vec![
                Command::CreateWidget(center, WidgetType::Center),
                Command::CreateWidget(
                    center,
                    WidgetType::Text("This is a text at the center".to_string()),
                ),
                Command::SetMainWidget(center),
                Command::AddChild {
                    parent_widget_id: center,
                    widget_placement: None,
                    child_widget_id: text,
                },
            ])
            .unwrap();

        App { widget_manager }
    }
}

impl PietApplication for App {
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
    run(Box::new(App::new()), "center", (400.0, 200.0).into());
}
