use std::ops::{Add, Div, Index, Mul, Sub};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point::new(self.x * other.x, self.y * other.y)
    }
}

impl Div for Point {
    type Output = Point;

    fn div(self, other: Point) -> Point {
        Point::new(self.x / other.x, self.y / other.y)
    }
}

impl Add<i32> for Point {
    type Output = Point;

    fn add(self, other: i32) -> Point {
        Point::new(self.x + other, self.y + other)
    }
}

impl Sub<i32> for Point {
    type Output = Point;

    fn sub(self, other: i32) -> Point {
        Point::new(self.x - other, self.y - other)
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, other: i32) -> Point {
        Point::new(self.x * other, self.y * other)
    }
}

impl Div<i32> for Point {
    type Output = Point;

    fn div(self, other: i32) -> Point {
        Point::new(self.x / other, self.y / other)
    }
}
