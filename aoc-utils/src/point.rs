use std::ops::Add;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

pub trait PointExt {
    fn neighbours(self) -> [Self; 8]
    where
        Self: Sized;

    fn cardinal_neighbours(self) -> [Self; 4]
    where
        Self: Sized;
}

impl PointExt for Point {
    fn neighbours(self) -> [Point; 8] {
        [
            self + Point::new(-1, -1), // north-west
            self + Point::new(0, -1),  // north
            self + Point::new(1, -1),  // north-east
            self + Point::new(-1, 0),  // west
            self + Point::new(1, 0),   // east
            self + Point::new(-1, 1),  // south-west
            self + Point::new(0, 1),   // south
            self + Point::new(1, 1),   // south-east
        ]
    }
    fn cardinal_neighbours(self) -> [Point; 4] {
        [
            self + Point::new(0, -1), // north
            self + Point::new(1, 0),  // east
            self + Point::new(0, 1),  // south
            self + Point::new(-1, 0), // west
        ]
    }
}

#[cfg(test)]
mod test {
    use crate::point::{Point, PointExt};

    #[test]
    fn add_point_test() {
        assert_eq!(Point::new(0, 3), Point::new(0, 1) + Point::new(0, 2));
        assert_eq!(Point::new(-1, -4), Point::new(0, 0) + Point::new(-1, -4));
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
            ],
            Point::new(1, 1).neighbours()
        )
    }

    #[test]
    fn cardinal_neighboursneighbour_test() {
        assert_eq!(
            [
                Point::new(-3, 3),
                Point::new(-2, 4),
                Point::new(-3, 5),
                Point::new(-4, 4),
            ],
            Point::new(-3, 4).cardinal_neighbours()
        )
    }
}
