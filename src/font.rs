use druid_shell::piet::{
    Color, FontFamily, FontWeight, PietText, PietTextLayout, Text, TextAttribute,
    TextLayoutBuilder, TextStorage,
};

///
#[derive(Clone, Debug)]
pub struct Font {
    pub font_color: Color,
    pub font_family: FontFamily,
    pub font_size: f64,
    pub font_weight: FontWeight,
    pub has_strikethrough: bool,
    pub has_underline: bool,
}

impl Default for Font {
    fn default() -> Self {
        Font {
            font_color: Color::rgb8(255, 255, 255),
            font_family: FontFamily::SYSTEM_UI,
            font_size: 14.0,
            font_weight: FontWeight::default(),
            has_strikethrough: false,
            has_underline: false,
        }
    }
}

impl Font {
    ///
    pub fn text_layout(&self, text: impl TextStorage) -> PietTextLayout {
        let mut piet_text = PietText::new_with_unique_state();

        piet_text
            .new_text_layout(text)
            .default_attribute(TextAttribute::Weight(self.font_weight))
            .default_attribute(TextAttribute::Underline(self.has_underline))
            .default_attribute(TextAttribute::Strikethrough(self.has_strikethrough))
            .font(self.font_family.clone(), self.font_size)
            .text_color(self.font_color.clone())
            .build()
            .unwrap()
    }
}
