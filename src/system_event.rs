use druid_shell::MouseEvent;

///
#[derive(Clone, Debug, PartialEq)]
pub enum SystemEvent {
    MouseDown(MouseEvent),
    MouseMove(MouseEvent),
    MouseUp(MouseEvent),
}
