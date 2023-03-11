use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    ///
    pub const ZERO: Point = Point::new(0.0, 0.0);

    ///
    pub const fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

// =================================================================================================

impl Add<(f64, f64)> for Point {
    type Output = Point;

    #[inline]
    fn add(self, (x, y): (f64, f64)) -> Self {
        Point::new(self.x + x, self.y + y)
    }
}

impl AddAssign<(f64, f64)> for Point {
    #[inline]
    fn add_assign(&mut self, (x, y): (f64, f64)) {
        *self = Point::new(self.x + x, self.y + y)
    }
}

impl Add<Point> for Point {
    type Output = Point;

    #[inline]
    fn add(self, point: Point) -> Self {
        Point::new(self.x + point.x, self.y + point.y)
    }
}

impl AddAssign<Point> for Point {
    #[inline]
    fn add_assign(&mut self, point: Point) {
        *self = Point::new(self.x + point.x, self.y + point.y)
    }
}

// =================================================================================================

impl Sub<(f64, f64)> for Point {
    type Output = Point;

    #[inline]
    fn sub(self, (x, y): (f64, f64)) -> Self {
        Point::new(self.x - x, self.y - y)
    }
}

impl SubAssign<(f64, f64)> for Point {
    #[inline]
    fn sub_assign(&mut self, (x, y): (f64, f64)) {
        *self = Point::new(self.x - x, self.y - y)
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    #[inline]
    fn sub(self, point: Point) -> Self {
        Point::new(self.x - point.x, self.y - point.y)
    }
}

impl SubAssign<Point> for Point {
    #[inline]
    fn sub_assign(&mut self, point: Point) {
        *self = Point::new(self.x - point.x, self.y - point.y)
    }
}
