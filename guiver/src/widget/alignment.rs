///
#[derive(Clone, Copy, Debug, Default)]
pub enum HorizontalAlignment {
    #[default]
    Center,
    Left,
    Right,
}

// =================================================================================================

///
#[derive(Clone, Copy, Debug, Default)]
pub enum VerticalAlignment {
    Bottom,
    #[default]
    Middle,
    Top,
}
