use crate::window_event_handler::WindowEventHandler;
use crate::Event;
use druid_shell::kurbo::Size;
use druid_shell::piet::Piet;
use druid_shell::{Clipboard, Region, WindowBuilder};

///
pub trait PietApplication {
    ///
    fn handle_event(&mut self, event: &Event);

    ///
    fn paint(&mut self, piet: &mut Piet, region: &Region);

    ///
    fn resize(&mut self, size: Size);

    ///
    fn set_clipboard(&mut self, clipboard: Clipboard);
}

///
pub fn run(mut application: Box<dyn PietApplication>, title: impl Into<String>, size: Size) {
    // Create a druid shell application.
    let druid_shell_application = druid_shell::Application::new().unwrap();

    // Set the global clipboard.
    application.set_clipboard(druid_shell_application.clipboard());

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
