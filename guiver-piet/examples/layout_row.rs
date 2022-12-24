use guiver::{Command, Size, WidgetManager};
use guiver_piet::{run, Application, Clipboard, Event, Piet, PietWidgetManager, Region};

pub(crate) struct App {
    widget_manager: PietWidgetManager<()>,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = PietWidgetManager::new();

        // Create the widgets.
        let padding = widget_manager.new_padding();
        let row = widget_manager.new_row();
        let placeholder1 = widget_manager.new_placeholder(Size::new(100.0, 50.0));
        let placeholder2 = widget_manager.new_placeholder(Size::new(100.0, 50.0));
        let placeholder3 = widget_manager.new_placeholder(Size::new(100.0, 50.0));

        // Compose the widgets.
        widget_manager
            .handle_commands(vec![
                Command::SetMainWidget(padding),
                Command::AddChild {
                    parent_widget_id: padding,
                    widget_placement: None,
                    child_widget_id: row,
                },
                Command::AddChild {
                    parent_widget_id: row,
                    widget_placement: None,
                    child_widget_id: placeholder1,
                },
                Command::AddChild {
                    parent_widget_id: row,
                    widget_placement: None,
                    child_widget_id: placeholder2,
                },
                Command::AddChild {
                    parent_widget_id: row,
                    widget_placement: None,
                    child_widget_id: placeholder3,
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
    run(Box::new(App::new()), "row", (400.0, 200.0).into());
}
