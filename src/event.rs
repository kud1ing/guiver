use druid_shell::{KeyEvent, MouseEvent};

///
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    KeyDown(KeyEvent),
    KeyUp(KeyEvent),
    MouseDown(MouseEvent),
    MouseMove(MouseEvent),
    MouseUp(MouseEvent),
}
