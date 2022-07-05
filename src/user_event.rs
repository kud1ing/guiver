use druid_shell::MouseEvent;

///
#[derive(Clone, Debug, PartialEq)]
pub enum UserEvent {
    MouseDown(MouseEvent),
    MouseMove(MouseEvent),
    MouseUp(MouseEvent),
}
