use guiver::Size;
use guiver_piet::{
    run, Clipboard, Command, Event, Piet, PietApplication, Region, WidgetManager, WidgetType,
};

pub(crate) struct App {
    widget_manager: WidgetManager<()>,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = WidgetManager::new();

        let padding = widget_manager.widget_id_provider().next_widget_id();
        let placeholder = widget_manager.widget_id_provider().next_widget_id();

        widget_manager
            .handle_commands(vec![
                // Create the widgets.
                // =================================================================================
                Command::CreateWidget(padding, WidgetType::Padding),
                Command::CreateWidget(
                    placeholder,
                    WidgetType::Placeholder {
                        maximum_size: Size::new(100.0, 50.0),
                    },
                ),
                // Compose the widgets.
                // =================================================================================
                Command::SetMainWidget(padding),
                Command::AddChild {
                    parent_widget_id: padding,
                    widget_placement: None,
                    child_widget_id: placeholder,
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
    run(Box::new(App::new()), "padding", (400.0, 200.0).into());
}
