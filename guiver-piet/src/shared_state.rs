use druid_shell::piet::PietText;
use guiver::font::piet_text;

///
pub struct SharedState {
    piet_text: PietText,
}

impl SharedState {
    ///
    pub fn new() -> Self {
        SharedState {
            piet_text: piet_text(),
        }
    }

    ///
    pub fn piet_text(&mut self) -> &mut PietText {
        &mut self.piet_text
    }
}
