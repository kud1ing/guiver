use crate::{Point, Size};

///
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Rect {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

impl Rect {
    ///
    pub const ZERO: Rect = Rect::new(0.0, 0.0, 0.0, 0.0);

    ///
    pub const fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Rect {
        Rect { x0, y0, x1, y1 }
    }

    ///
    pub fn abs(&self) -> Rect {
        let Rect { x0, y0, x1, y1 } = *self;
        Rect::new(x0.min(x1), y0.min(y1), x0.max(x1), y0.max(y1))
    }

    ///
    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.x0 && x < self.x1 && y >= self.y0 && y < self.y1
    }

    ///
    pub fn from_origin_size(origin: impl Into<Point>, size: Size) -> Rect {
        let origin = origin.into();
        Rect::new(
            origin.x,
            origin.y,
            origin.x + size.width,
            origin.y + size.height,
        )
        .abs()
    }

    ///
    pub fn height(&self) -> f64 {
        self.y1 - self.y0
    }

    ///
    pub fn origin(&self) -> Point {
        Point::new(self.x0, self.y0)
    }

    ///
    pub fn size(&self) -> Size {
        Size::new(self.width(), self.height())
    }

    ///
    pub fn width(&self) -> f64 {
        self.x1 - self.x0
    }

    ///
    pub fn with_origin(self, origin: impl Into<Point>) -> Rect {
        Rect::from_origin_size(origin, self.size())
    }

    ///
    pub fn with_size(self, size: Size) -> Rect {
        Rect::from_origin_size(self.origin(), size)
    }
}
