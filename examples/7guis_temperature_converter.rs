/**
This implements the "Counter" task from [7GUIs](https://eugenkiss.github.io/7guis/tasks/).
 */
use guiver::{
    run, Application, Clipboard, Command, Event, HorizontalAlignment, Piet, Region, Size,
    WidgetEvent, WidgetId, WidgetManager,
};

pub(crate) struct App {
    clipboard: Option<Clipboard>,
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
        let text_input_celsius = widget_manager.new_text_input("", 50.0);
        let text1 = widget_manager.new_text("Celsius =");
        let text_input_fahrenheit = widget_manager.new_text_input("", 50.0);
        let text2 = widget_manager.new_text("Fahrenheit");

        // Compose the widgets.
        widget_manager
            .send_commands(vec![
                Command::SetMainWidget(padding),
                Command::AddChild(padding, None, row),
                Command::AddChild(row, None, text_input_celsius),
                Command::AddChild(row, None, text1),
                Command::AddChild(row, None, text_input_fahrenheit),
                Command::AddChild(row, None, text2),
                //
                Command::SetHasFocus(text_input_celsius, true),
                Command::SetHorizontalAlignment(text_input_celsius, HorizontalAlignment::Right),
                Command::SetHorizontalAlignment(text_input_fahrenheit, HorizontalAlignment::Left),
            ])
            .unwrap();

        App {
            clipboard: None,
            text_input_celsius,
            text_input_fahrenheit,
            widget_manager,
        }
    }
}

fn celsius_from_fahrenheit(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}

fn fahrenheit_from_celsius(celsius: f32) -> f32 {
    celsius * (9.0 / 5.0) + 32.0
}

impl Application for App {
    fn handle_event(&mut self, event: &Event) {
        // Handle the system event, possibly create widget events.
        let widget_events = self
            .widget_manager
            .handle_event(event, self.clipboard.as_mut())
            .unwrap();

        // Iterate over the generated widget events.
        for widget_event in widget_events {
            match widget_event {
                WidgetEvent::ValueChanged(widget_id, value) => {
                    if widget_id == self.text_input_celsius {
                        // The given value is a string.
                        if let Some(string) = value.downcast_ref::<String>() {
                            // The string is empty.
                            if string.trim().is_empty() {
                                self.widget_manager
                                    .send_command(Command::SetValue(
                                        self.text_input_fahrenheit,
                                        Box::new("".to_string()),
                                    ))
                                    .unwrap();
                            }
                            // The string could be parsed as a float
                            else if let Ok(celsius) = string.parse::<f32>() {
                                self.widget_manager
                                    .send_command(Command::SetValue(
                                        self.text_input_fahrenheit,
                                        Box::new(format!(
                                            "{:.0}",
                                            fahrenheit_from_celsius(celsius)
                                        )),
                                    ))
                                    .unwrap();
                            }
                        }
                    } else if widget_id == self.text_input_fahrenheit {
                        // The given value is a string.
                        if let Some(string) = value.downcast_ref::<String>() {
                            // The string is empty.
                            if string.trim().is_empty() {
                                self.widget_manager
                                    .send_command(Command::SetValue(
                                        self.text_input_celsius,
                                        Box::new("".to_string()),
                                    ))
                                    .unwrap();
                            }
                            // The string could be parsed as a float
                            else if let Ok(fahrenheit) = string.parse::<f32>() {
                                self.widget_manager
                                    .send_command(Command::SetValue(
                                        self.text_input_celsius,
                                        Box::new(format!(
                                            "{:.0}",
                                            celsius_from_fahrenheit(fahrenheit)
                                        )),
                                    ))
                                    .unwrap();
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn paint(&mut self, piet: &mut Piet, region: &Region) {
        self.widget_manager.paint(piet, region).unwrap();
    }

    fn resize(&mut self, size: Size) {
        self.widget_manager.resize(size);
    }

    fn set_clipboard(&mut self, clipboard: Clipboard) {
        self.clipboard = Some(clipboard)
    }
}

pub fn main() {
    run(
        Box::new(App::new()),
        "7GUIs: Temperature Converter",
        (400.0, 80.0).into(),
    );
}
