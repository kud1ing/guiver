use guiver::{Command, Size, WidgetManager, WidgetType};
use guiver_piet::{run, Clipboard, Event, Piet, PietApplication, PietWidgetManager, Region};

pub(crate) struct App {
    widget_manager: PietWidgetManager<()>,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = PietWidgetManager::new();

        let padding = widget_manager.widget_id_provider().next_widget_id();
        let text = widget_manager.widget_id_provider().next_widget_id();
        let row = widget_manager.widget_id_provider().next_widget_id();
        let expanded1 = widget_manager.widget_id_provider().next_widget_id();
        let expanded2 = widget_manager.widget_id_provider().next_widget_id();
        let placeholder1 = widget_manager.widget_id_provider().next_widget_id();
        let placeholder2 = widget_manager.widget_id_provider().next_widget_id();

        widget_manager
            .handle_commands(vec![
                // Create the widgets.
                // =================================================================================
                Command::CreateWidget(padding, WidgetType::Padding),
                Command::CreateWidget(
                    text,
                    WidgetType::Text("The placeholders are expanded".to_string()),
                ),
                Command::CreateWidget(row, WidgetType::Row),
                Command::CreateWidget(expanded1, WidgetType::Expanded { flex_factor: 1 }),
                Command::CreateWidget(expanded2, WidgetType::Expanded { flex_factor: 1 }),
                Command::CreateWidget(
                    placeholder1,
                    WidgetType::Placeholder {
                        maximum_size: Size::new(100.0, 50.0),
                    },
                ),
                Command::CreateWidget(
                    placeholder2,
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
                    child_widget_id: row,
                },
                Command::AddChild {
                    parent_widget_id: row,
                    widget_placement: None,
                    child_widget_id: expanded1,
                },
                Command::AddChild {
                    parent_widget_id: row,
                    widget_placement: None,
                    child_widget_id: text,
                },
                Command::AddChild {
                    parent_widget_id: row,
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
    run(Box::new(App::new()), "expanded row", (300.0, 150.0).into());
}
