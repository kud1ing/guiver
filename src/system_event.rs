use druid_shell::{KeyEvent, MouseEvent};

///
#[derive(Clone, Debug, PartialEq)]
pub enum SystemEvent {
    KeyDown(KeyEvent),
    KeyUp(KeyEvent),
    MouseDown(MouseEvent),
    MouseMove(MouseEvent),
    MouseUp(MouseEvent),
}
