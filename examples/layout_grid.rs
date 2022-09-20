use guiver::widget::layout::GridColumnProperties;
use guiver::widget::layout::GridRowProperties;
use guiver::{run, Application, Clipboard, Command, Event, Piet, Region, Size, WidgetManager};

pub(crate) struct App {
    widget_manager: WidgetManager,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = WidgetManager::new();

        // Create the widgets.
        let padding = widget_manager.new_padding();
        let grid = widget_manager.new_grid(
            GridColumnProperties::default(),
            GridRowProperties::default(),
        );
        let placeholder1 = widget_manager.new_placeholder(Size::new(100.0, 50.0));
        let placeholder2 = widget_manager.new_placeholder(Size::new(100.0, 50.0));
        let placeholder3 = widget_manager.new_placeholder(Size::new(100.0, 50.0));

        // Compose the widgets.
        widget_manager
            .send_commands(vec![
                Command::SetMainWidget(padding),
                Command::AppendChild(padding, grid),
                Command::AppendChild(grid, placeholder1),
                Command::AppendChild(grid, placeholder2),
                Command::AppendChild(grid, placeholder3),
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
    run(Box::new(App::new()), "grid", (400.0, 200.0).into());
}
