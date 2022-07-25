/**
This implements the "Counter" task from [7GUIs](https://eugenkiss.github.io/7guis/tasks/).
 */
use druid_shell::kurbo::Size;
use druid_shell::piet::Piet;
use druid_shell::Region;
use guiver::{run, Command, WidgetEvent, WidgetId, WidgetManager};
use guiver::{Application, SystemEvent};

pub(crate) struct App {
    temperature_celsius: f32,
    temperature_fahrenheit: f32,
    text_input_celsius: WidgetId,
    text_input_fahrenheit: WidgetId,
    widget_manager: WidgetManager,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = WidgetManager::new();

        // Create the widgets.
        let padding = widget_manager.new_padding();
        let row = widget_manager.new_row();
        let text_input_celsius = widget_manager.new_text_input("", 100.0);
        let text1 = widget_manager.new_text("Celsius =");
        let text_input_fahrenheit = widget_manager.new_text_input("", 100.0);
        let text2 = widget_manager.new_text("Fahrenheit");

        // Compose the widgets.
        widget_manager
            .send_commands(vec![
                Command::SetMainWidget(padding),
                Command::AppendChild(padding, row),
                Command::AppendChild(row, text_input_celsius),
                Command::AppendChild(row, text1),
                Command::AppendChild(row, text_input_fahrenheit),
                Command::AppendChild(row, text2),
            ])
            .unwrap();

        App {
            temperature_celsius: 0.0,
            temperature_fahrenheit: 0.0,
            text_input_celsius,
            text_input_fahrenheit,
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
                WidgetEvent::Clicked(_) => {}
                WidgetEvent::GotFocus(_) => {}
                WidgetEvent::LostFocus(_) => {}
                WidgetEvent::ValueChanged(widget_id, value) => {
                    if widget_id == self.text_input_celsius {
                        // TODO
                        println!("Celsius: {:?}", &value);
                    } else if widget_id == self.text_input_fahrenheit {
                        // TODO
                        println!("Fahrenheit: {:?}", &value);
                    }
                }
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
    run(
        Box::new(App::new()),
        "7GUIs: Temperature Converter",
        (400.0, 80.0).into(),
    );
}
