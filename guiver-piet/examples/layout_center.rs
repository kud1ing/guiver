use guiver::{Command, Size, WidgetManager};
use guiver_piet::{run, Application, Clipboard, Event, Piet, PietWidgetManager, Region};

pub(crate) struct App {
    widget_manager: PietWidgetManager<()>,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = PietWidgetManager::new();

        // Create the widgets.
        let center = widget_manager.new_center();
        let text = widget_manager.new_text("This is a text at the center");

        // Compose the widgets.
        widget_manager
            .handle_commands(vec![
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
    run(Box::new(App::new()), "center", (400.0, 200.0).into());
}
