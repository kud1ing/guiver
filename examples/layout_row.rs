/**
This implements the "Counter" task from [7GUIs](https://eugenkiss.github.io/7guis/tasks/).
*/
use druid_shell::kurbo::Size;
use druid_shell::piet::Piet;
use druid_shell::Region;
use guiver::widget::layout::{Padding, Row};
use guiver::widget::Label;
use guiver::{run, WidgetEvent, WidgetId, WidgetManager, WidgetManagerCommand};
use guiver::{Application, SystemEvent};

pub(crate) struct App {
    widget_manager: WidgetManager,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = WidgetManager::new();

        let padding = widget_manager.new_padding(15.0, 15.0, 15.0, 15.0);
        let row = widget_manager.new_row(15.0);
        let placeholder1 = widget_manager.new_placeholder();
        let placeholder2 = widget_manager.new_placeholder();
        let placeholder3 = widget_manager.new_placeholder();

        let _ = widget_manager.send_commands(vec![
            WidgetManagerCommand::SetMainWidget(padding),
            WidgetManagerCommand::AppendChild(padding, row),
            WidgetManagerCommand::AppendChild(row, placeholder1),
            WidgetManagerCommand::AppendChild(row, placeholder2),
            WidgetManagerCommand::AppendChild(row, placeholder3),
        ]);

        App { widget_manager }
    }
}

impl Application for App {
    fn handle_system_event(&mut self, system_event: &SystemEvent) {
        // Do nothing.
    }

    fn paint(&mut self, piet: &mut Piet, region: &Region) {
        // Paint the main widget.
        self.widget_manager.paint(piet, region);
    }

    fn resize(&mut self, size: Size) {
        // Resize the main widget.
        self.widget_manager.resize(size);
    }
}

pub fn main() {
    run(Box::new(App::new()), "row", (400.0, 300.0).into());
}
