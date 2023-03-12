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

        let layout_padding = widget_manager.widget_id_provider().next_widget_id();
        let layout_column = widget_manager.widget_id_provider().next_widget_id();
        let layout_row = widget_manager.widget_id_provider().next_widget_id();
        let layout_expanded1 = widget_manager.widget_id_provider().next_widget_id();
        let layout_expanded2 = widget_manager.widget_id_provider().next_widget_id();

        let text = widget_manager.widget_id_provider().next_widget_id();
        let placeholder1 = widget_manager.widget_id_provider().next_widget_id();
        let placeholder2 = widget_manager.widget_id_provider().next_widget_id();

        widget_manager
            .handle_commands(vec![
                // Create the widgets.
                // =================================================================================
                Command::CreateWidget(layout_padding, WidgetType::LayoutPadding),
                Command::CreateWidget(layout_column, WidgetType::LayoutColumn),
                Command::CreateWidget(
                    text,
                    WidgetType::Text("The placeholders are expanded".to_string()),
                ),
                Command::CreateWidget(layout_row, WidgetType::LayoutRow),
                Command::CreateWidget(
                    layout_expanded1,
                    WidgetType::LayoutExpanded { flex_factor: 1 },
                ),
                Command::CreateWidget(
                    layout_expanded2,
                    WidgetType::LayoutExpanded { flex_factor: 1 },
                ),
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
                Command::SetMainWidget(layout_padding),
                Command::AddChild {
                    parent_widget_id: layout_padding,
                    widget_placement: None,
                    child_widget_id: layout_column,
                },
                Command::AddChild {
                    parent_widget_id: layout_column,
                    widget_placement: None,
                    child_widget_id: layout_row,
                },
                Command::AddChild {
                    parent_widget_id: layout_row,
                    widget_placement: None,
                    child_widget_id: text,
                },
                Command::AddChild {
                    parent_widget_id: layout_row,
                    widget_placement: None,
                    child_widget_id: layout_expanded1,
                },
                Command::AddChild {
                    parent_widget_id: layout_expanded1,
                    widget_placement: None,
                    child_widget_id: placeholder1,
                },
                Command::AddChild {
                    parent_widget_id: layout_column,
                    widget_placement: None,
                    child_widget_id: layout_expanded2,
                },
                Command::AddChild {
                    parent_widget_id: layout_expanded2,
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

    fn resize(&mut self, size: kurbo::Size) {
        self.widget_manager.resize(size);
    }

    fn set_clipboard(&mut self, _clipboard: Clipboard) {}
}

pub fn main() {
    run(Box::new(App::new()), "expanded", (300.0, 150.0).into());
}
