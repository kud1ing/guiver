use druid_shell::kurbo;
use guiver::widget_manager::WidgetManager;
use guiver_piet::{
    run, Clipboard, Command, Event, Piet, PietApplication, PietWidgetManager, Region, WidgetType,
};

pub(crate) struct App {
    widget_manager: PietWidgetManager<()>,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = PietWidgetManager::new();

        let layout_center = widget_manager.widget_id_provider().next_widget_id();
        let text = widget_manager.widget_id_provider().next_widget_id();

        widget_manager
            .handle_commands(vec![
                // Create the widgets.
                // =================================================================================
                Command::CreateWidget(layout_center, WidgetType::LayoutCenter),
                Command::CreateWidget(
                    layout_center,
                    WidgetType::Text("This is a text at the center".to_string()),
                ),
                // Compose the widgets.
                // =================================================================================
                Command::SetMainWidget(layout_center),
                Command::AddChild {
                    parent_widget_id: layout_center,
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

    fn resize(&mut self, size: kurbo::Size) {
        self.widget_manager.resize(size);
    }

    fn set_clipboard(&mut self, _clipboard: Clipboard) {}
}

pub fn main() {
    run(Box::new(App::new()), "center", (400.0, 200.0).into());
}
