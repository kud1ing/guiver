use druid_shell::piet::PietText;

#[cfg(any(target_os = "linux", target_os = "openbsd", target_os = "freebsd"))]
use druid_shell::piet::CairoText;

#[cfg(target_os = "windows")]
use druid_shell::piet::{D2DText, DwriteFactory};

#[cfg(any(target_os = "linux", target_os = "openbsd", target_os = "freebsd"))]
pub fn piet_text() -> PietText {
    CairoText::new()
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub fn piet_text() -> PietText {
    PietText::new_with_unique_state()
}

#[cfg(target_os = "windows")]
pub fn piet_text() -> PietText {
    let dwrite = DwriteFactory::new().unwrap();
    D2DText::new_with_shared_fonts(dwrite, None)
}

///
pub struct PietSharedState {
    piet_text: PietText,
}

impl PietSharedState {
    ///
    pub fn new() -> Self {
        PietSharedState {
            piet_text: piet_text(),
        }
    }

    ///
    pub fn piet_text(&mut self) -> &mut PietText {
        &mut self.piet_text
    }
}
