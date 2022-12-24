use druid_shell::{KeyEvent, MouseEvent};

///
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    ClipboardPaste(String),
    KeyDown(KeyEvent),
    KeyUp(KeyEvent),
    MouseDown(MouseEvent),
    MouseMove(MouseEvent),
    MouseUp(MouseEvent),
    RequestClose,
}
