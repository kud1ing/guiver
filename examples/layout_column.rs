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

        // Create the widgets.
        let padding = widget_manager.new_padding(15.0, 15.0, 15.0, 15.0);
        let row = widget_manager.new_row(15.0);
        let column1 = widget_manager.new_column(15.0);
        let column2 = widget_manager.new_column(15.0);
        let placeholder1 = widget_manager.new_placeholder();
        let placeholder2 = widget_manager.new_placeholder();
        let placeholder3 = widget_manager.new_placeholder();
        let placeholder4 = widget_manager.new_placeholder();
        let placeholder5 = widget_manager.new_placeholder();
        let placeholder6 = widget_manager.new_placeholder();

        // Compose the widgets.
        let _ = widget_manager.send_commands(vec![
            WidgetManagerCommand::SetMainWidget(padding),
            WidgetManagerCommand::AppendChild(padding, row),
            WidgetManagerCommand::AppendChild(row, column1),
            WidgetManagerCommand::AppendChild(column1, placeholder1),
            WidgetManagerCommand::AppendChild(column1, placeholder2),
            WidgetManagerCommand::AppendChild(column1, placeholder3),
            WidgetManagerCommand::AppendChild(row, column2),
            WidgetManagerCommand::AppendChild(column2, placeholder4),
            WidgetManagerCommand::AppendChild(column2, placeholder5),
            WidgetManagerCommand::AppendChild(row, placeholder6),
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
    run(Box::new(App::new()), "column", (400.0, 200.0).into());
}
