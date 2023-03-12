use crate::Size;

/// Constraints for the size of a widget.
#[derive(Clone, Copy, Debug, Default)]
pub struct SizeConstraints {
    maximum: Size,
    minimum: Size,
}

impl SizeConstraints {
    ///
    pub fn new(minimum: Size, maximum: Size) -> Self {
        SizeConstraints { maximum, minimum }
    }

    /// Creates size constraints, where the minimum size is zero and the maximum size equals the
    /// given size.
    pub fn loose(size: Size) -> Self {
        SizeConstraints {
            maximum: size,
            minimum: Size::ZERO,
        }
    }

    ///
    pub fn maximum(&self) -> &Size {
        &self.maximum
    }

    ///
    pub fn minimum(&self) -> &Size {
        &self.minimum
    }

    /// Shrinks the current size constraints by the given delta size.
    pub fn shrink(&self, delta: impl Into<Size>) -> Self {
        let delta = delta.into();

        let minimum = Size::new(
            (self.minimum().width - delta.width).max(0.),
            (self.minimum().height - delta.height).max(0.),
        );
        let maximum = Size::new(
            (self.maximum().width - delta.width).max(0.),
            (self.maximum().height - delta.height).max(0.),
        );

        SizeConstraints::new(minimum, maximum)
    }

    /// Creates size constraints, where both the minimum and the maximum size equal the given size.
    pub fn tight(size: Size) -> Self {
        SizeConstraints {
            maximum: size,
            minimum: size,
        }
    }

    ///
    pub fn unbounded() -> Self {
        SizeConstraints {
            maximum: Size::new(f64::INFINITY, f64::INFINITY),
            minimum: Size::ZERO,
        }
    }
}
