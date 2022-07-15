use druid_shell::kurbo::Size;
/**
This implements the "Counter" task from [7GUIs](https://eugenkiss.github.io/7guis/tasks/).
*/
use guiver::{run, WidgetEvent, WidgetId, WidgetManager};

use druid_shell::piet::Piet;
use druid_shell::Region;
use guiver::widget::layout::{Padding, Row};
use guiver::widget::{Label, WidgetCommand};
use guiver::{Application, UserEvent};

pub(crate) struct App {
    counter: u32,
    id_counter_button: WidgetId,
    id_counter_label: WidgetId,
    widget_manager: WidgetManager,
}

impl App {
    pub(crate) fn new() -> Self {
        let no_widget_id = 0;
        let widget_id_counter_button = 1;
        let widget_id_counter_label = 2;

        let mut widget_manager = WidgetManager::new(Box::new(Padding::new(
            no_widget_id,
            15.0,
            15.0,
            15.0,
            15.0,
            Some(Box::new(Row::new(
                no_widget_id,
                10.0,
                vec![
                    Box::new(Label::new(widget_id_counter_label, "0")),
                    Box::new(Label::new(widget_id_counter_button, "Count")),
                ],
            ))),
        )));

        let _x = widget_manager.new_label("0");
        let _y = widget_manager.new_label("Count");

        App {
            counter: 0,
            id_counter_button: widget_id_counter_button,
            id_counter_label: widget_id_counter_label,
            widget_manager,
        }
    }
}

impl Application for App {
    fn handle_user_event(&mut self, user_event: &UserEvent) {
        // Handle the user events.
        let widget_events = self.widget_manager.handle_event(user_event);

        // Iterate over the widget events.
        for widget_event in widget_events {
            match widget_event {
                WidgetEvent::Clicked(widget_id) => {
                    // The counter button was clicked.
                    if widget_id == self.id_counter_button {
                        // Increase the counter.
                        self.counter += 1;

                        // Update the counter label.
                        self.widget_manager.send_command(
                            self.id_counter_label,
                            WidgetCommand::SetValue(Box::new(format!("{}", self.counter))),
                        );
                    }
                }
                WidgetEvent::ValueChanged(_) => {}
            }
        }
    }

    fn paint(&mut self, piet: &mut Piet, region: &Region) {
        // Paint the widget tree.
        self.widget_manager.paint(piet, region);
    }

    fn resize(&mut self, size: Size) {
        // Resize the widget tree.
        self.widget_manager.resize(size);
    }
}

pub fn main() {
    run(
        Box::new(App::new()),
        "guiver 7GUIs: Counter",
        (400.0, 80.0).into(),
    );
}
