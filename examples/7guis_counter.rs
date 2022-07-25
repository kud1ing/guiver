/**
This implements the "Counter" task from [7GUIs](https://eugenkiss.github.io/7guis/tasks/).
*/
use druid_shell::kurbo::Size;
use druid_shell::piet::Piet;
use druid_shell::Region;
use guiver::{run, Command, WidgetEvent, WidgetId, WidgetManager};
use guiver::{Application, SystemEvent};

pub(crate) struct App {
    counter: u32,
    counter_button: WidgetId,
    counter_label: WidgetId,
    widget_manager: WidgetManager,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = WidgetManager::new();

        // Create the widgets.
        let padding = widget_manager.new_padding();
        let row = widget_manager.new_row();
        let counter_label = widget_manager.new_text("0");
        let counter_button = widget_manager.new_text_button("Count");

        // Compose the widgets.
        widget_manager
            .send_commands(vec![
                Command::SetMainWidget(padding),
                Command::AppendChild(padding, row),
                Command::AppendChild(row, counter_label),
                Command::AppendChild(row, counter_button),
            ])
            .unwrap();

        App {
            counter: 0,
            counter_button,
            counter_label,
            widget_manager,
        }
    }
}

impl Application for App {
    fn handle_system_event(&mut self, system_event: &SystemEvent) {
        // Handle the system event, possibly create widget events.
        let widget_events = self.widget_manager.handle_event(system_event).unwrap();

        // Iterate over the generated widget events.
        for widget_event in widget_events {
            match widget_event {
                WidgetEvent::Clicked(widget_id) => {
                    // The counter button was clicked.
                    if widget_id == self.counter_button {
                        // Increase the counter.
                        self.counter += 1;

                        // Update the counter text.
                        self.widget_manager
                            .send_command(Command::SetValue(
                                self.counter_label,
                                Box::new(format!("{}", self.counter)),
                            ))
                            .unwrap();
                    }
                }
                _ => {}
            }
        }
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
    run(Box::new(App::new()), "7GUIs: Counter", (400.0, 80.0).into());
}
