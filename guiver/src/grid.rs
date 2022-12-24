use crate::{HorizontalAlignment, VerticalAlignment};

///
pub struct GridColumnProperties {
    pub flex_factor: u16,
    pub horizontal_alignment: HorizontalAlignment,
    pub minimum_width: f64,
    pub spacing: f64,
}

impl Default for GridColumnProperties {
    fn default() -> Self {
        GridColumnProperties {
            flex_factor: 0,
            horizontal_alignment: HorizontalAlignment::Center,
            minimum_width: 0.0,
            spacing: 10.0,
        }
    }
}

// =================================================================================================

///
pub struct GridRowProperties {
    pub flex_factor: u16,
    pub minimum_height: f64,
    pub spacing: f64,
    pub vertical_alignment: VerticalAlignment,
}

impl Default for GridRowProperties {
    fn default() -> Self {
        GridRowProperties {
            flex_factor: 0,
            minimum_height: 0.0,
            spacing: 10.0,
            vertical_alignment: VerticalAlignment::Middle,
        }
    }
}
