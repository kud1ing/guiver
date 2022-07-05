use crate::widget_manager::WidgetManager;
use crate::{Application, UserEvent};
use druid_shell::kurbo::Size;
use druid_shell::piet::Piet;
use druid_shell::{
    FileDialogToken, FileInfo, IdleToken, KeyEvent, MouseEvent, Region, Scale, TimerToken,
    WinHandler, WindowHandle,
};
use std::any::Any;

///
pub struct WidgetAndApplicationManager {
    ///
    application: Box<dyn Application>,
    ///
    handle: WindowHandle,
    ///
    size: Size,
    /// The widget manager.
    widget_manager: WidgetManager,
}

impl WidgetAndApplicationManager {
    ///
    pub fn new(mut application: Box<dyn Application>) -> Self {
        let mut widget_manager = WidgetManager::new();

        // Set up the application with the widget manager.
        application.setup(&mut widget_manager);

        WidgetAndApplicationManager {
            application,
            handle: WindowHandle::default(),
            size: Size::default(),
            widget_manager,
        }
    }

    ///
    pub fn handle_user_event(&mut self, user_event: &UserEvent) {
        let widget_ids_and_events = self.widget_manager.handle_user_event(user_event);

        for (widget_id, widget_event) in widget_ids_and_events {
            // Let the application handle the current widget event.
            self.application.handle_widget_event(
                &mut self.widget_manager,
                widget_id,
                &widget_event,
            );
        }

        // Let the application handle the user event, too.
        self.application.handle_user_event(user_event);
    }
}

impl WinHandler for WidgetAndApplicationManager {
    fn connect(&mut self, handle: &WindowHandle) {
        self.handle = handle.clone();
    }

    fn size(&mut self, size: Size) {
        self.size = size;
    }

    fn scale(&mut self, _scale: Scale) {
        // TODO: Handle the event.
        // self.handle_user_event();

        self.handle.invalidate_rect(self.size.to_rect());
    }

    fn prepare_paint(&mut self) {}

    fn paint(&mut self, piet: &mut Piet, region: &Region) {
        // First let the application paint.
        self.application.paint(piet, region);

        // Paint the widgets.
        self.widget_manager.paint(piet, region);
    }

    fn rebuild_resources(&mut self) {}

    fn command(&mut self, _id: u32) {}

    fn save_as(&mut self, _token: FileDialogToken, _file: Option<FileInfo>) {}

    fn open_file(&mut self, _token: FileDialogToken, _file: Option<FileInfo>) {}

    fn key_down(&mut self, _event: KeyEvent) -> bool {
        // TODO: Handle the event.
        // self.handle_user_event();

        self.handle.invalidate_rect(self.size.to_rect());
        false
    }

    fn key_up(&mut self, _event: KeyEvent) {
        // TODO: Handle the event.
        // self.handle_user_event();

        self.handle.invalidate_rect(self.size.to_rect());
    }

    fn wheel(&mut self, _event: &MouseEvent) {
        // TODO: Handle the event.
        // self.handle_user_event();

        self.handle.invalidate_rect(self.size.to_rect());
    }

    fn zoom(&mut self, _delta: f64) {
        // TODO: Handle the event.
        // self.handle_user_event();

        self.handle.invalidate_rect(self.size.to_rect());
    }

    fn mouse_move(&mut self, event: &MouseEvent) {
        // Handle the mouse move event.
        self.handle_user_event(&UserEvent::MouseMove(event.clone()));

        self.handle.invalidate_rect(self.size.to_rect());
    }

    fn mouse_down(&mut self, event: &MouseEvent) {
        // Handle the mouse down event.
        self.handle_user_event(&UserEvent::MouseDown(event.clone()));

        self.handle.invalidate_rect(self.size.to_rect());
    }

    fn mouse_up(&mut self, event: &MouseEvent) {
        // Handle the mouse up event.
        self.handle_user_event(&UserEvent::MouseUp(event.clone()));

        self.handle.invalidate_rect(self.size.to_rect());
    }

    fn mouse_leave(&mut self) {
        // TODO: Handle the event.
        // self.handle_user_event();

        self.handle.invalidate_rect(self.size.to_rect());
    }

    fn timer(&mut self, _token: TimerToken) {}

    fn got_focus(&mut self) {
        // TODO: Handle the event.
        // self.handle_user_event();

        self.handle.invalidate_rect(self.size.to_rect());
    }

    fn lost_focus(&mut self) {
        // TODO: Handle the event.
        // self.handle_user_event();

        self.handle.invalidate_rect(self.size.to_rect());
    }

    fn request_close(&mut self) {}

    fn destroy(&mut self) {}

    fn idle(&mut self, _token: IdleToken) {}

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
