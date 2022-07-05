use crate::widget_and_application_manager::WidgetAndApplicationManager;
use crate::widget_manager::WidgetManager;
use crate::{UserEvent, WidgetEvent, WidgetId};
use druid_shell::kurbo::Size;
use druid_shell::piet::Piet;
use druid_shell::{Region, WindowBuilder};

///
pub trait Application {
    ///
    fn handle_user_event(&mut self, user_event: &UserEvent);

    ///
    fn handle_widget_event(
        &mut self,
        widget_manager: &mut WidgetManager,
        widget_id: WidgetId,
        widget_event: &WidgetEvent,
    );

    ///
    fn paint(&mut self, piet: &mut Piet, region: &Region);

    ///
    fn setup(&mut self, widget_manager: &mut WidgetManager);
}

///
pub fn run(application: Box<dyn Application>, title: impl Into<String>, size: Size) {
    // Create a druid shell application.
    let druid_shell_application = druid_shell::Application::new().unwrap();

    // Create a window builder.
    let mut window_builder = WindowBuilder::new(druid_shell_application.clone());
    window_builder.set_handler(Box::new(WidgetAndApplicationManager::new(application)));
    window_builder.set_title(title);
    window_builder.set_size(size);

    // Create a window and show it.
    let window_handle = window_builder.build().unwrap();
    window_handle.show();

    druid_shell_application.run(None);
}
