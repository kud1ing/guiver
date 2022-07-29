use druid_shell::piet::{
    Color, FontFamily, PietText, PietTextLayout, Text, TextLayoutBuilder, TextStorage,
};

///
#[derive(Clone, Debug)]
pub struct Font {
    pub color: Color,
    pub font_family: FontFamily,
    pub font_size: f64,
}

impl Default for Font {
    fn default() -> Self {
        Font {
            color: Color::rgb8(255, 255, 255),
            font_family: FontFamily::SYSTEM_UI,
            font_size: 14.0,
        }
    }
}

impl Font {
    ///
    pub fn text_layout(&self, text: impl TextStorage) -> PietTextLayout {
        let mut piet_text = PietText::new_with_unique_state();

        piet_text
            .new_text_layout(text)
            .font(self.font_family.clone(), self.font_size)
            .text_color(self.color.clone())
            .build()
            .unwrap()
    }
}
