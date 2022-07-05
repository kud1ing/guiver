use guiver::run;

use druid_shell::kurbo::Rect;
use druid_shell::piet::{Color, FontFamily, Piet};
use druid_shell::Region;
use guiver::widget::{Label, LabelCommand, Placeholder, WidgetCommand};
use guiver::{Application, Font, UserEvent, WidgetEvent, WidgetId, WidgetManager};

pub(crate) struct App {
    counter: u32,
    label1: WidgetId,
    label2: WidgetId,
}

impl App {
    pub(crate) fn new() -> Self {
        App {
            counter: 0,
            label1: 0,
            label2: 0,
        }
    }
}

impl Application for App {
    fn handle_user_event(&mut self, _user_event: &UserEvent) {}

    fn handle_widget_event(
        &mut self,
        widget_manager: &mut WidgetManager,
        widget_id: WidgetId,
        widget_event: &WidgetEvent,
    ) {
        // Something was clicked.
        if widget_event == &WidgetEvent::Clicked {
            self.counter += 1;

            // The first label was clicked.
            if widget_id == self.label1 {
                widget_manager.send_commands_to_widget(
                    self.label1,
                    vec![
                        Box::new(LabelCommand::SetText(format!(
                            "This label was clicked ({} clicks so far)",
                            self.counter
                        ))),
                        Box::new(LabelCommand::SetPostion((50.0, 50.0))),
                        Box::new(LabelCommand::SetFont(Font {
                            color: Color::rgb8(0, 255, 100),
                            font_family: FontFamily::SYSTEM_UI,
                            font_size: 20.0,
                        })),
                    ],
                );

                widget_manager.send_commands_to_widget(
                    self.label2,
                    vec![
                        Box::new(LabelCommand::SetText(
                            "The label above was clicked".to_string(),
                        )),
                        Box::new(LabelCommand::SetPostion((200.0, 100.0))),
                        Box::new(LabelCommand::SetFont(Font {
                            color: Color::rgb8(200, 200, 200),
                            font_family: FontFamily::SYSTEM_UI,
                            font_size: 12.0,
                        })),
                    ],
                );
                return;
            }

            // The second label was clicked.
            if widget_id == self.label2 {
                widget_manager.send_commands_to_widget(
                    self.label1,
                    vec![
                        Box::new(LabelCommand::SetText(
                            "The label below was clicked".to_string(),
                        )),
                        Box::new(LabelCommand::SetPostion((200.0, 50.0))),
                        Box::new(LabelCommand::SetFont(Font {
                            color: Color::rgb8(200, 200, 200),
                            font_family: FontFamily::SYSTEM_UI,
                            font_size: 12.0,
                        })),
                    ],
                );

                widget_manager.send_commands_to_widget(
                    self.label2,
                    vec![
                        Box::new(LabelCommand::SetText(format!(
                            "This label was clicked ({} clicks so far)",
                            self.counter
                        ))),
                        Box::new(LabelCommand::SetPostion((50.0, 100.0))),
                        Box::new(LabelCommand::SetFont(Font {
                            color: Color::rgb8(255, 100, 0),
                            font_family: FontFamily::SYSTEM_UI,
                            font_size: 20.0,
                        })),
                    ],
                );
            }
            return;
        }
    }

    fn paint(&mut self, _piet: &mut Piet, _region: &Region) {}

    ///
    fn setup(&mut self, widget_manager: &mut WidgetManager) {
        self.label1 = widget_manager.add_widget(Box::new(Label::new(
            "This is a label",
            vec![
                Box::new(LabelCommand::SetPostion((50.0, 50.0))),
                Box::new(LabelCommand::SetFont(Font {
                    color: Color::rgb8(255, 255, 255),
                    font_family: FontFamily::SYSTEM_UI,
                    font_size: 14.0,
                })),
            ],
        )));

        self.label2 = widget_manager.add_widget(Box::new(Label::new(
            "This is a big label",
            vec![
                Box::new(LabelCommand::SetPostion((50.0, 100.0))),
                Box::new(LabelCommand::SetFont(Font {
                    color: Color::rgb8(0, 100, 255),
                    font_family: FontFamily::SYSTEM_UI,
                    font_size: 20.0,
                })),
            ],
        )));

        widget_manager.add_widget(Box::new(Placeholder::new(vec![Box::new(
            WidgetCommand::SetRectangle(Rect::new(450.0, 50.0, 550.0, 300.0)),
        )])));
    }
}

pub fn main() {
    run(Box::new(App::new()), "guiver", (800.0, 400.0).into());
}
