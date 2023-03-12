/**
This implements the "Flight Booker" task from [7GUIs](https://eugenkiss.github.io/7guis/tasks/).
*/
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

        let dropdown_box = widget_manager.widget_id_provider().next_widget_id();
        let text_input_start_date = widget_manager.widget_id_provider().next_widget_id();
        let text_input_return_date = widget_manager.widget_id_provider().next_widget_id();
        let book_button = widget_manager.widget_id_provider().next_widget_id();

        widget_manager
            .handle_commands(vec![
                // Create the widgets.
                // =================================================================================
                Command::CreateWidget(layout_padding, WidgetType::LayoutPadding),
                Command::CreateWidget(layout_column, WidgetType::LayoutColumn),
                Command::CreateWidget(
                    dropdown_box,
                    WidgetType::Placeholder {
                        maximum_size: Size::new(200.0, 50.0),
                    },
                ),
                Command::CreateWidget(
                    text_input_start_date,
                    WidgetType::TextInput {
                        text: "".to_string(),
                        width: 100.0,
                    },
                ),
                Command::CreateWidget(
                    text_input_return_date,
                    WidgetType::TextInput {
                        text: "".to_string(),
                        width: 100.0,
                    },
                ),
                Command::CreateWidget(book_button, WidgetType::TextButton("Book".to_string())),
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
                    child_widget_id: dropdown_box,
                },
                Command::AddChild {
                    parent_widget_id: layout_column,
                    widget_placement: None,
                    child_widget_id: text_input_start_date,
                },
                Command::AddChild {
                    parent_widget_id: layout_column,
                    widget_placement: None,
                    child_widget_id: text_input_return_date,
                },
                Command::AddChild {
                    parent_widget_id: layout_column,
                    widget_placement: None,
                    child_widget_id: book_button,
                },
            ])
            .unwrap();

        App { widget_manager }
    }
}

impl PietApplication for App {
    fn handle_event(&mut self, event: &Event) {
        let _widget_events = self.widget_manager.handle_event(event, None).unwrap();

        // TODO
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
    run(
        Box::new(App::new()),
        "7GUIs: Flight Booker",
        (300.0, 150.0).into(),
    );
}
