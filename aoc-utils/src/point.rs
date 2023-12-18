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

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Point::new(x as i64, y as i64)
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point::new(x as i64, y as i64)
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

#[cfg(test)]
mod test {
    use crate::point::Point;

    #[test]
    fn add_point_test() {
        assert_eq!(Point::new(0, 3), Point::new(0, 1) + Point::new(0, 2));
        assert_eq!(Point::new(-1, -4), Point::new(0, 0) + Point::new(-1, -4));
    }

    #[test]
    fn sub_point_test() {
        assert_eq!(Point::new(0, -1), Point::new(0, 1) - Point::new(0, 2));
        assert_eq!(Point::new(1, 4), Point::new(0, 0) - Point::new(-1, -4));
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
                Point::new(2, 2)
            ]
            .sort(),
            Point::new(1, 1).neighbours().sort()
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
