use crate::{Application, SystemEvent};
use druid_shell::kurbo::Size;
use druid_shell::piet::Piet;
use druid_shell::{
    FileDialogToken, FileInfo, IdleToken, KeyEvent, MouseEvent, Region, Scale, TimerToken,
    WinHandler, WindowHandle,
};
use std::any::Any;

///
pub struct WindowEventHandler {
    ///
    application: Box<dyn Application>,
    ///
    window_handle: WindowHandle,
    ///
    window_size: Size,
}

impl WindowEventHandler {
    ///
    pub fn new(application: Box<dyn Application>) -> Self {
        WindowEventHandler {
            application,
            window_handle: WindowHandle::default(),
            window_size: Size::default(),
        }
    }
}

impl WinHandler for WindowEventHandler {
    fn connect(&mut self, handle: &WindowHandle) {
        self.window_handle = handle.clone();
    }

    fn size(&mut self, size: Size) {
        self.window_size = size;
        self.application.resize(size);
    }

    fn scale(&mut self, _scale: Scale) {
        // TODO: Handle the event.

        self.window_handle
            .invalidate_rect(self.window_size.to_rect());
    }

    fn prepare_paint(&mut self) {}

    fn paint(&mut self, piet: &mut Piet, region: &Region) {
        // First let the application paint.
        self.application.paint(piet, region);
    }

    fn rebuild_resources(&mut self) {}

    fn command(&mut self, _id: u32) {}

    fn save_as(&mut self, _token: FileDialogToken, _file: Option<FileInfo>) {}

    fn open_file(&mut self, _token: FileDialogToken, _file: Option<FileInfo>) {}

    fn key_down(&mut self, event: KeyEvent) -> bool {
        // Handle the key down event.
        self.application
            .handle_system_event(&SystemEvent::KeyDown(event.clone()));

        self.window_handle
            .invalidate_rect(self.window_size.to_rect());
        false
    }

    fn key_up(&mut self, event: KeyEvent) {
        // Handle the key up event.
        self.application
            .handle_system_event(&SystemEvent::KeyUp(event.clone()));

        self.window_handle
            .invalidate_rect(self.window_size.to_rect());
    }

    fn wheel(&mut self, _event: &MouseEvent) {
        // TODO: Handle the event.

        self.window_handle
            .invalidate_rect(self.window_size.to_rect());
    }

    fn zoom(&mut self, _delta: f64) {
        // TODO: Handle the event.

        self.window_handle
            .invalidate_rect(self.window_size.to_rect());
    }

    fn mouse_move(&mut self, event: &MouseEvent) {
        // Handle the mouse move event.
        self.application
            .handle_system_event(&SystemEvent::MouseMove(event.clone()));

        self.window_handle
            .invalidate_rect(self.window_size.to_rect());
    }

    fn mouse_down(&mut self, event: &MouseEvent) {
        // Handle the mouse down event.
        self.application
            .handle_system_event(&SystemEvent::MouseDown(event.clone()));

        self.window_handle
            .invalidate_rect(self.window_size.to_rect());
    }

    fn mouse_up(&mut self, event: &MouseEvent) {
        // Handle the mouse up event.
        self.application
            .handle_system_event(&SystemEvent::MouseUp(event.clone()));

        self.window_handle
            .invalidate_rect(self.window_size.to_rect());
    }

    fn mouse_leave(&mut self) {
        // TODO: Handle the event.

        self.window_handle
            .invalidate_rect(self.window_size.to_rect());
    }

    fn timer(&mut self, _token: TimerToken) {}

    fn got_focus(&mut self) {
        // TODO: Handle the event.

        self.window_handle
            .invalidate_rect(self.window_size.to_rect());
    }

    fn lost_focus(&mut self) {
        // TODO: Handle the event.

        self.window_handle
            .invalidate_rect(self.window_size.to_rect());
    }

    fn request_close(&mut self) {}

    fn destroy(&mut self) {}

    fn idle(&mut self, _token: IdleToken) {}

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
