///
#[derive(Clone, Copy, Debug)]
pub enum HorizontalAlignment {
    Center,
    Left,
    Right,
}

impl Default for HorizontalAlignment {
    fn default() -> Self {
        HorizontalAlignment::Center
    }
}

// =================================================================================================

///
#[derive(Clone, Copy, Debug)]
pub enum VerticalAlignment {
    Bottom,
    Middle,
    Top,
}

impl Default for VerticalAlignment {
    fn default() -> Self {
        VerticalAlignment::Middle
    }
}
