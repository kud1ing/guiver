use guiver::{Command, Size, WidgetManager};
/**
This implements the "Flight Booker" task from [7GUIs](https://eugenkiss.github.io/7guis/tasks/).
*/
use guiver_piet::{
    run, Clipboard, Color, Event, PaintBrush, Piet, PietApplication, PietWidgetManager, Region,
};

pub(crate) struct App {
    widget_manager: PietWidgetManager<()>,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = PietWidgetManager::new();

        // Create the widgets.
        let padding = widget_manager.new_padding();
        let column = widget_manager.new_column();
        let dropdown_box = widget_manager.new_placeholder(Size::new(200.0, 50.0));
        let text_input_start_date = widget_manager.new_text_input("".to_string(), 100.0);
        let text_input_return_date = widget_manager.new_text_input("".to_string(), 100.0);
        let book_button = widget_manager.new_text_button("Book".to_string());

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
                    child_widget_id: dropdown_box,
                },
                Command::AddChild {
                    parent_widget_id: column,
                    widget_placement: None,
                    child_widget_id: text_input_start_date,
                },
                Command::AddChild {
                    parent_widget_id: column,
                    widget_placement: None,
                    child_widget_id: text_input_return_date,
                },
                Command::AddChild {
                    parent_widget_id: column,
                    widget_placement: None,
                    child_widget_id: book_button,
                },
                //
                // TODO: remove
                Command::SetFill(
                    text_input_start_date,
                    Some(PaintBrush::Color(Color::rgb8(255, 0, 0))),
                ),
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

    fn resize(&mut self, size: Size) {
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
