use std::ops::{Add, AddAssign, Sub, SubAssign};

///
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    pub height: f64,
    pub width: f64,
}

impl Size {
    ///
    pub const ZERO: Size = Size::new(0.0, 0.0);

    ///
    pub const fn new(width: f64, height: f64) -> Self {
        Self { height, width }
    }

    ///
    pub fn clamp(self, min: Size, max: Size) -> Self {
        let width = self.width.max(min.width).min(max.width);
        let height = self.height.max(min.height).min(max.height);
        Size { height, width }
    }
}

impl Add<Size> for Size {
    type Output = Size;

    #[inline]
    fn add(self, other: Size) -> Size {
        Size {
            height: self.height + other.height,
            width: self.width + other.width,
        }
    }
}

impl AddAssign<Size> for Size {
    #[inline]
    fn add_assign(&mut self, other: Size) {
        *self = *self + other;
    }
}

impl Sub<Size> for Size {
    type Output = Size;

    #[inline]
    fn sub(self, other: Size) -> Size {
        Size {
            height: self.height - other.height,
            width: self.width - other.width,
        }
    }
}

impl SubAssign<Size> for Size {
    #[inline]
    fn sub_assign(&mut self, other: Size) {
        *self = *self - other;
    }
}
