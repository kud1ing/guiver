use guiver::{GridColumnProperties, GridRowProperties, Size, WidgetPlacement};
use guiver_piet::{
    run, Clipboard, Command, Event, Piet, PietApplication, Region, WidgetManager, WidgetType,
};

pub(crate) struct App {
    widget_manager: WidgetManager<()>,
}

impl App {
    pub(crate) fn new() -> Self {
        let mut widget_manager = WidgetManager::new();

        let padding = widget_manager.widget_id_provider().next_widget_id();
        let grid = widget_manager.widget_id_provider().next_widget_id();
        let placeholder1 = widget_manager.widget_id_provider().next_widget_id();
        let placeholder2 = widget_manager.widget_id_provider().next_widget_id();
        let placeholder3 = widget_manager.widget_id_provider().next_widget_id();
        let placeholder4 = widget_manager.widget_id_provider().next_widget_id();
        let placeholder5 = widget_manager.widget_id_provider().next_widget_id();
        let placeholder6 = widget_manager.widget_id_provider().next_widget_id();

        widget_manager
            .handle_commands(vec![
                // Create the widgets.
                // =================================================================================
                Command::CreateWidget(padding, WidgetType::Padding),
                Command::CreateWidget(
                    grid,
                    WidgetType::Grid {
                        column_properties: GridColumnProperties::default(),
                        row_properties: GridRowProperties::default(),
                    },
                ),
                Command::CreateWidget(
                    placeholder1,
                    WidgetType::Placeholder {
                        maximum_size: Size::new(70.0, 30.0),
                    },
                ),
                Command::CreateWidget(
                    placeholder2,
                    WidgetType::Placeholder {
                        maximum_size: Size::new(70.0, 30.0),
                    },
                ),
                Command::CreateWidget(
                    placeholder3,
                    WidgetType::Placeholder {
                        maximum_size: Size::new(70.0, 30.0),
                    },
                ),
                Command::CreateWidget(
                    placeholder4,
                    WidgetType::Placeholder {
                        maximum_size: Size::new(70.0, 30.0),
                    },
                ),
                Command::CreateWidget(
                    placeholder5,
                    WidgetType::Placeholder {
                        maximum_size: Size::new(70.0, 30.0),
                    },
                ),
                Command::CreateWidget(
                    placeholder6,
                    WidgetType::Placeholder {
                        maximum_size: Size::new(70.0, 30.0),
                    },
                ),
                // Compose the widgets.
                // =================================================================================
                Command::SetMainWidget(padding),
                Command::AddChild {
                    parent_widget_id: padding,
                    widget_placement: None,
                    child_widget_id: grid,
                },
                //
                Command::AddChild {
                    parent_widget_id: grid,
                    widget_placement: Some(WidgetPlacement::Grid {
                        column_index: 0,
                        row_index: 1,
                    }),
                    child_widget_id: placeholder1,
                },
                //
                Command::AddChild {
                    parent_widget_id: grid,
                    widget_placement: Some(WidgetPlacement::Grid {
                        column_index: 1,
                        row_index: 0,
                    }),
                    child_widget_id: placeholder2,
                },
                Command::AddChild {
                    parent_widget_id: grid,
                    widget_placement: Some(WidgetPlacement::Grid {
                        column_index: 1,
                        row_index: 1,
                    }),
                    child_widget_id: placeholder3,
                },
                Command::AddChild {
                    parent_widget_id: grid,
                    widget_placement: Some(WidgetPlacement::Grid {
                        column_index: 1,
                        row_index: 2,
                    }),
                    child_widget_id: placeholder4,
                },
                //
                Command::AddChild {
                    parent_widget_id: grid,
                    widget_placement: Some(WidgetPlacement::Grid {
                        column_index: 2,
                        row_index: 0,
                    }),
                    child_widget_id: placeholder5,
                },
                Command::AddChild {
                    parent_widget_id: grid,
                    widget_placement: Some(WidgetPlacement::Grid {
                        column_index: 2,
                        row_index: 2,
                    }),
                    child_widget_id: placeholder6,
                },
            ])
            .unwrap();

        App { widget_manager }
    }
}

impl PietApplication for App {
    fn handle_event(&mut self, _system_event_event: &Event) {}

    fn paint(&mut self, piet: &mut Piet, region: &Region) {
        self.widget_manager.paint(piet, region).unwrap();
    }

    fn resize(&mut self, size: Size) {
        self.widget_manager.resize(size);
    }
    fn set_clipboard(&mut self, _clipboard: Clipboard) {}
}

pub fn main() {
    run(Box::new(App::new()), "grid", (400.0, 200.0).into());
}
