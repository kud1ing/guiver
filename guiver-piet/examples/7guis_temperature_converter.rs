/**
This implements the "Counter" task from [7GUIs](https://eugenkiss.github.io/7guis/tasks/).
 */
use druid_shell::Region;
use guiver::{
    Command, HorizontalAlignment, Size, WidgetEvent, WidgetEventType, WidgetId, WidgetManager,
};
use guiver_piet::{run, Clipboard, Event, Piet, PietApplication, PietWidgetManager};

///
#[derive(Clone)]
enum CustomEvent {
    ConvertFromCtoF,
    ConvertFromFtoC,
}

///
pub(crate) struct App {
    clipboard: Option<Clipboard>,
    text_input_celsius: WidgetId,
    text_input_fahrenheit: WidgetId,
    widget_manager: PietWidgetManager<CustomEvent>,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = PietWidgetManager::new();

        // Create the widgets.
        let padding = widget_manager.new_padding();
        let row = widget_manager.new_row();
        let text_input_celsius = widget_manager.new_text_input("".to_string(), 50.0);
        let text1 = widget_manager.new_text("Celsius =".to_string());
        let text_input_fahrenheit = widget_manager.new_text_input("".to_string(), 50.0);
        let text2 = widget_manager.new_text("Fahrenheit".to_string());

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
                    child_widget_id: text_input_celsius,
                },
                Command::AddChild {
                    parent_widget_id: row,
                    widget_placement: None,
                    child_widget_id: text1,
                },
                Command::AddChild {
                    parent_widget_id: row,
                    widget_placement: None,
                    child_widget_id: text_input_fahrenheit,
                },
                Command::AddChild {
                    parent_widget_id: row,
                    widget_placement: None,
                    child_widget_id: text2,
                },
                //
                Command::SetHasFocus(text_input_celsius, true),
                Command::SetHorizontalAlignment(text_input_celsius, HorizontalAlignment::Right),
                Command::SetHorizontalAlignment(text_input_fahrenheit, HorizontalAlignment::Left),
                //
                Command::AddEventObservation(
                    text_input_celsius,
                    WidgetEventType::ValueChanged,
                    CustomEvent::ConvertFromCtoF,
                ),
                Command::AddEventObservation(
                    text_input_fahrenheit,
                    WidgetEventType::ValueChanged,
                    CustomEvent::ConvertFromFtoC,
                ),
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

impl PietApplication for App {
    fn handle_event(&mut self, event: &Event) {
        // Handle the given event, possibly creating widget events.
        let widget_events = self
            .widget_manager
            .handle_event(event, self.clipboard.as_mut())
            .unwrap();

        // Iterate over the generated widget events.
        for widget_event in widget_events {
            match widget_event {
                WidgetEvent::Custom(CustomEvent::ConvertFromCtoF) => {
                    // Try to get the widget's value.
                    let value = self
                        .widget_manager
                        .value(self.text_input_celsius)
                        .unwrap()
                        .unwrap();

                    // The given value is a string.
                    if let Some(string) = value.downcast_ref::<String>() {
                        // The string is empty.
                        if string.trim().is_empty() {
                            self.widget_manager
                                .handle_command(Command::SetValue(
                                    self.text_input_fahrenheit,
                                    Box::new("".to_string()),
                                ))
                                .unwrap();
                        }
                        // The string could be parsed as a float
                        else if let Ok(celsius) = string.parse::<f32>() {
                            self.widget_manager
                                .handle_command(Command::SetValue(
                                    self.text_input_fahrenheit,
                                    Box::new(format!("{:.0}", fahrenheit_from_celsius(celsius))),
                                ))
                                .unwrap();
                        }
                    }
                }
                WidgetEvent::Custom(CustomEvent::ConvertFromFtoC) => {
                    // Try to get the widget's value.
                    let value = self
                        .widget_manager
                        .value(self.text_input_fahrenheit)
                        .unwrap()
                        .unwrap();

                    // The given value is a string.
                    if let Some(string) = value.downcast_ref::<String>() {
                        // The string is empty.
                        if string.trim().is_empty() {
                            self.widget_manager
                                .handle_command(Command::SetValue(
                                    self.text_input_celsius,
                                    Box::new("".to_string()),
                                ))
                                .unwrap();
                        }
                        // The string could be parsed as a float
                        else if let Ok(fahrenheit) = string.parse::<f32>() {
                            self.widget_manager
                                .handle_command(Command::SetValue(
                                    self.text_input_celsius,
                                    Box::new(format!("{:.0}", celsius_from_fahrenheit(fahrenheit))),
                                ))
                                .unwrap();
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
