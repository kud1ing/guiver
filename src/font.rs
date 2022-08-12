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
}

impl Default for Font {
    fn default() -> Self {
        Font {
            font_color: Color::rgb8(255, 255, 255),
            font_family: FontFamily::SYSTEM_UI,
            font_size: 14.0,
            font_weight: FontWeight::default(),
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
            .font(self.font_family.clone(), self.font_size)
            .text_color(self.font_color.clone())
            .build()
            .unwrap()
    }
}
