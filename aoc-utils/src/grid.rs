use std::ops::Index;

use crate::point::Point;

pub struct Grid<T> {
    pub width: i64, // to remain consistent with point
    pub height: i64,
    pub data: Vec<T>,
}

impl Grid<i64> {
    pub fn parse(input: &str) -> Self {
        let raw = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as i64)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let width = raw[0].len() as i64;
        let height = raw.len() as i64;
        let data = raw
            .iter()
            .flatten()
            .map(|e| e.to_owned())
            .collect::<Vec<_>>();

        Grid {
            width,
            height,
            data,
        }
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }

    pub fn neighbours(&self, point: Point) -> Vec<(Point, T)> {
        point
            .cardinal()
            .iter()
            .filter(|&p| self.contains(*p))
            .map(|&p| (p, self[p]))
            .collect::<Vec<_>>()
    }
}

impl<T: Copy + PartialEq> Index<Point> for Grid<T> {
    type Output = T;

    // points outside the grid will throw an index out of bounds error
    fn index(&self, point: Point) -> &Self::Output {
        &self.data[(self.width * point.y + point.x) as usize]
    }
}

#[cfg(test)]
mod test {
    use crate::{grid::Grid, point::Point};

    const INPUT: &str = r"12345
67891
23456
78912
34567";

    #[test]
    fn index_test() {
        let grid = Grid::parse(INPUT);
        assert_eq!(1, grid[Point::new(0, 0)]);
        assert_eq!(4, grid[Point::new(3, 0)]);
        assert_eq!(7, grid[Point::new(0, 3)]);
    }

    #[test]
    fn contains_test() {
        let grid = Grid::parse(INPUT);

        assert!(grid.contains(Point::new(0, 0)));
        assert!(grid.contains(Point::new(3, 2)));
        assert!(grid.contains(Point::new(4, 4)));
        assert!(!grid.contains(Point::new(-1, 0)));
        assert!(!grid.contains(Point::new(5, 5)));
    }
}
