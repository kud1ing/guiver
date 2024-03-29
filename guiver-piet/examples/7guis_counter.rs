/**
This implements the "Counter" task from [7GUIs](https://eugenkiss.github.io/7guis/tasks/).
*/
use druid_shell::kurbo;
use guiver::widget_manager::WidgetManager;
use guiver::{WidgetEventType, WidgetId};
use guiver_piet::{run, Command, Piet, PietWidgetManager, Region, WidgetType};
use guiver_piet::{Clipboard, Event, PietApplication};

///
#[derive(Clone)]
enum CustomEvent {
    Count,
}

///
pub(crate) struct App {
    counter: u32,
    counter_text: WidgetId,
    widget_manager: PietWidgetManager<CustomEvent>,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = PietWidgetManager::new();

        let layout_padding = widget_manager.widget_id_provider().next_widget_id();
        let layout_row = widget_manager.widget_id_provider().next_widget_id();

        let counter_text = widget_manager.widget_id_provider().next_widget_id();
        let counter_button = widget_manager.widget_id_provider().next_widget_id();

        widget_manager
            .handle_commands(vec![
                // Create the widgets.
                // =================================================================================
                Command::CreateWidget(layout_padding, WidgetType::LayoutPadding),
                Command::CreateWidget(layout_row, WidgetType::LayoutRow),
                Command::CreateWidget(counter_text, WidgetType::Text("0".to_string())),
                Command::CreateWidget(counter_button, WidgetType::TextButton("Count".to_string())),
                // Compose the widgets.
                // =================================================================================
                Command::SetMainWidget(layout_padding),
                Command::AddChild {
                    parent_widget_id: layout_padding,
                    widget_placement: None,
                    child_widget_id: layout_row,
                },
                Command::AddChild {
                    parent_widget_id: layout_row,
                    widget_placement: None,
                    child_widget_id: counter_text,
                },
                Command::AddChild {
                    parent_widget_id: layout_row,
                    widget_placement: None,
                    child_widget_id: counter_button,
                },
                // Add event observations.
                // =================================================================================
                Command::AddEventObservation(
                    counter_button,
                    WidgetEventType::Clicked,
                    CustomEvent::Count,
                ),
            ])
            .unwrap();

        App {
            counter: 0,
            counter_text,
            widget_manager,
        }
    }
}

impl PietApplication for App {
    fn handle_event(&mut self, event: &Event) {
        // Handle the given event, possibly creating widget events.
        let widget_events = self.widget_manager.handle_event(event, None).unwrap();

        // Iterate over the generated widget events.
        for widget_event in widget_events {
            match widget_event {
                CustomEvent::Count => {
                    // Increase the counter.
                    self.counter += 1;

                    // Update the counter text.
                    self.widget_manager
                        .handle_command(Command::SetValue(
                            self.counter_text,
                            Box::new(format!("{}", self.counter)),
                        ))
                        .unwrap();
                }
            }
        }
    }

    fn paint(&mut self, piet: &mut Piet, region: &Region) {
        self.widget_manager.paint(piet, region).unwrap();
    }

    fn resize(&mut self, size: kurbo::Size) {
        self.widget_manager.resize(size);
    }
    fn set_clipboard(&mut self, _clipboard: Clipboard) {}
}

pub fn main() {
    run(Box::new(App::new()), "7GUIs: Counter", (400.0, 80.0).into());
}
