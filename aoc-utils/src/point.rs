use core::fmt;
use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

pub const ORIGIN: Point = Point::new(0, 0);
pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);

impl Point {
    pub const fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<usize> for Point {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self {
        Point::new(self.x * rhs as i64, self.y * rhs as i64)
    }
}

impl Point {
    pub fn neighbours(self) -> [Point; 8] {
        [
            self + UP,
            self + RIGHT,
            self + DOWN,
            self + LEFT,
            self + Point::new(-1, -1), // north-west
            self + Point::new(1, -1),  // north-east
            self + Point::new(-1, 1),  // south-west
            self + Point::new(1, 1),   // south-east
        ]
    }
    pub fn cardinal(self) -> [Point; 4] {
        [self + UP, self + RIGHT, self + DOWN, self + LEFT]
    }

    pub fn determinant(self, other: Self) -> i64 {
        self.x * other.y - self.y * other.x
    }

    pub fn manhattan(self, other: Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

/// Automatically generate from tuple implementation traits
macro_rules! to_point {
    ($($t:ty)*) => ($(
        impl From<($t, $t)> for Point {
            fn from((x, y): ($t, $t)) -> Self {
                Point {
                    x: x as i64,
                    y: y as i64,
                }
            }
        }
    )*)
}

to_point!(u8 u16 u32 usize i8 i16 i32 i64);

#[cfg(test)]
mod test {
    use crate::point::{Point, ORIGIN};

    #[test]
    fn add_point_test() {
        assert_eq!(Point::new(0, 2), ORIGIN + Point::new(0, 2));
        assert_eq!(Point::new(-1, -4), ORIGIN + Point::new(-1, -4));
    }

    #[test]
    fn sub_point_test() {
        assert_eq!(Point::new(0, -2), ORIGIN - Point::from((0, 2)));
        assert_eq!(Point::new(1, 4), ORIGIN - Point::from((-1, -4)));
    }

    #[test]
    fn neighbour_test() {
        assert_eq!(
            [
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(0, 1),
                Point::new(2, 1),
                Point::new(0, 2),
                Point::new(1, 2),
                Point::from((2, 2))
            ]
            .sort(),
            Point::from((1, 1)).neighbours().sort()
        )
    }

    #[test]
    fn cardinal_test() {
        assert_eq!(
            [
                Point::new(-3, 3),
                Point::new(-2, 4),
                Point::new(-3, 5),
                Point::new(-4, 4),
            ],
            Point::new(-3, 4).cardinal()
        )
    }
}
