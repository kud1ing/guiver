/**
This implements the "Counter" task from [7GUIs](https://eugenkiss.github.io/7guis/tasks/).
*/
use druid_shell::kurbo::Size;
use druid_shell::piet::Piet;
use druid_shell::Region;
use guiver::{run, Command, WidgetManager};
use guiver::{Application, SystemEvent};

pub(crate) struct App {
    widget_manager: WidgetManager,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = WidgetManager::new();

        // Create the widgets.
        let padding = widget_manager.new_padding();
        let placeholder = widget_manager.new_placeholder();

        // Compose the widgets.
        let _ = widget_manager.send_commands(vec![
            Command::SetMainWidget(padding),
            Command::AppendChild(padding, placeholder),
        ]);

        App { widget_manager }
    }
}

impl Application for App {
    fn handle_system_event(&mut self, _system_event: &SystemEvent) {
        // Do nothing.
    }

    fn paint(&mut self, piet: &mut Piet, region: &Region) {
        // Paint the main widget.
        self.widget_manager.paint(piet, region).unwrap();
    }

    fn resize(&mut self, size: Size) {
        // Resize the main widget.
        self.widget_manager.resize(size);
    }
}

pub fn main() {
    run(Box::new(App::new()), "padding", (400.0, 200.0).into());
}
