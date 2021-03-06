use crate::window_event_handler::WindowEventHandler;
use crate::SystemEvent;
use druid_shell::kurbo::Size;
use druid_shell::piet::Piet;
use druid_shell::{Region, WindowBuilder};

///
pub trait Application {
    ///
    fn handle_user_event(&mut self, user_event: &SystemEvent);

    ///
    fn paint(&mut self, piet: &mut Piet, region: &Region);

    ///
    fn resize(&mut self, size: Size);
}

///
pub fn run(mut application: Box<dyn Application>, title: impl Into<String>, size: Size) {
    // Create a druid shell application.
    let druid_shell_application = druid_shell::Application::new().unwrap();

    // Set the initial size.
    application.resize(size);

    // Create a window builder.
    let mut window_builder = WindowBuilder::new(druid_shell_application.clone());
    window_builder.set_handler(Box::new(WindowEventHandler::new(application)));
    window_builder.set_title(title);
    window_builder.set_size(size);

    // Create a window and show it.
    let window_handle = window_builder.build().unwrap();
    window_handle.show();

    druid_shell_application.run(None);
}
