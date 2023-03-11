use druid_shell::kurbo;
use guiver::widget_manager::WidgetManager;
use guiver::Size;
use guiver_piet::{
    run, Clipboard, Command, Event, Piet, PietApplication, PietWidgetManager, Region, WidgetType,
};

pub(crate) struct App {
    widget_manager: PietWidgetManager<()>,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = PietWidgetManager::new();

        // Create the widgets.
        let padding = widget_manager.widget_id_provider().next_widget_id();
        let column = widget_manager.widget_id_provider().next_widget_id();
        let placeholder1 = widget_manager.widget_id_provider().next_widget_id();
        let placeholder2 = widget_manager.widget_id_provider().next_widget_id();
        let placeholder3 = widget_manager.widget_id_provider().next_widget_id();

        // Compose the widgets.
        widget_manager
            .handle_commands(vec![
                Command::CreateWidget(padding, WidgetType::Padding),
                Command::CreateWidget(column, WidgetType::Column),
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
                Command::CreateWidget(
                    placeholder3,
                    WidgetType::Placeholder {
                        maximum_size: Size::new(100.0, 50.0),
                    },
                ),
                Command::SetMainWidget(padding),
                Command::AddChild {
                    parent_widget_id: padding,
                    widget_placement: None,
                    child_widget_id: column,
                },
                Command::AddChild {
                    parent_widget_id: column,
                    widget_placement: None,
                    child_widget_id: placeholder1,
                },
                Command::AddChild {
                    parent_widget_id: column,
                    widget_placement: None,
                    child_widget_id: placeholder2,
                },
                Command::AddChild {
                    parent_widget_id: column,
                    widget_placement: None,
                    child_widget_id: placeholder3,
                },
            ])
            .unwrap();

        App { widget_manager }
    }
}

impl PietApplication for App {
    fn handle_event(&mut self, event: &Event) {
        let _widget_events = self.widget_manager.handle_event(event, None);
    }

    fn paint(&mut self, piet: &mut Piet, region: &Region) {
        self.widget_manager.paint(piet, region).unwrap();
    }

    fn resize(&mut self, size: kurbo::Size) {
        self.widget_manager.resize(size);
    }

    fn set_clipboard(&mut self, _clipboard: Clipboard) {}
}

pub fn main() {
    run(Box::new(App::new()), "column", (400.0, 200.0).into());
}
