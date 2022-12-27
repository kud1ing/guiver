use guiver::{Command, Size, WidgetManager};
use guiver_piet::{run, Clipboard, Event, Piet, PietApplication, PietWidgetManager, Region};

pub(crate) struct App {
    widget_manager: PietWidgetManager<()>,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = PietWidgetManager::new();

        // Create the widgets.
        let padding = widget_manager.new_padding();
        let column = widget_manager.new_column();
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

    fn resize(&mut self, size: Size) {
        self.widget_manager.resize(size);
    }
    fn set_clipboard(&mut self, _clipboard: Clipboard) {}
}

pub fn main() {
    run(Box::new(App::new()), "column", (400.0, 200.0).into());
}
