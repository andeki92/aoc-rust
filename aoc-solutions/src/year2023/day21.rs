//! # Day 21: Step Counter

use std::collections::{HashSet, VecDeque};

use utils::{grid::Grid, point::Point};

pub fn input(raw: &str) -> (Grid<char>, Point) {
    let gardens = Grid::parse_char(raw);
    let start = gardens.find('S').unwrap();
    (gardens, start)
}

pub fn part_one((gardens, start): &(Grid<char>, Point)) -> usize {
    explore(gardens, start, 64)
}

pub fn part_two((gardens, start): &(Grid<char>, Point)) -> usize {
    expand(gardens, start, 26501365)
}

pub fn expand(garden: &Grid<char>, start: &Point, steps: i64) -> usize {
    // ensure the puzzle is a square
    assert!(garden.width == garden.height);
    let size = garden.width;

    // ensure we start in the middle
    assert!(start.x == start.y && start.x == size / 2);

    // ensure the edge of the puzzle is reachable with a whole number of grids,
    // offset by the initial half-grid
    assert!(steps % size == size / 2);

    let num_grids = (steps / size - 1) as usize;

    // floor division to nearest two and squared
    let odd = ((num_grids / 2 * 2) + 1).pow(2) as usize;

    // rounded up to the nearest two and squared
    let even = ((num_grids + 1) / 2 * 2).pow(2) as usize;

    let odd_points = explore(garden, start, size * 2 + 1);
    let even_points = explore(garden, start, size * 2);

    // calculate all the special - not fully-filled grids
    let corner_rs = size - 1; // remaining steps
    let corner_t = explore(garden, &Point::new(size - 1, start.y), corner_rs);
    let corner_r = explore(garden, &Point::new(start.x, 0), corner_rs);
    let corner_b = explore(garden, &Point::new(0, start.y), corner_rs);
    let corner_l = explore(garden, &Point::new(start.x, size - 1), corner_rs);

    // calculate outside small pieces
    let small_rs = size / 2 - 1; // remaning steps
    let small_tr = explore(garden, &Point::new(size - 1, 0), small_rs);
    let small_tl = explore(garden, &Point::new(size - 1, size - 1), small_rs);
    let small_br = explore(garden, &Point::new(0, 0), small_rs);
    let small_bl = explore(garden, &Point::new(0, size - 1), small_rs);

    // calculate outside large pieces
    let large_rs = size * 3 / 2 - 1; // remaning steps
    let large_tr = explore(garden, &Point::new(size - 1, 0), large_rs);
    let large_tl = explore(garden, &Point::new(size - 1, size - 1), large_rs);
    let large_br = explore(garden, &Point::new(0, 0), large_rs);
    let large_bl = explore(garden, &Point::new(0, size - 1), large_rs);

    odd * odd_points + // number of odd grids multiplied by the number of points in each odd grid
    even * even_points + // same for evens
    corner_t + corner_r + corner_b + corner_l +
    (num_grids + 1) * (small_tr + small_tl + small_br + small_bl) +
    (num_grids) * (large_tr + large_tl + large_br + large_bl)
}

pub fn explore(garden: &Grid<char>, start: &Point, steps: i64) -> usize {
    let mut sum = HashSet::new();
    let mut visisted: HashSet<Point> = HashSet::from([start.to_owned()]);

    let mut queue = VecDeque::from([(start.to_owned(), steps)]);

    while let Some((pos, steps)) = queue.pop_front() {
        if steps % 2 == 0 {
            sum.insert(pos);
        }

        if steps == 0 {
            continue; // empty the queue before exiting, but no need to calculate stuff
        }

        for neighbour in pos.cardinal() {
            if visisted.contains(&neighbour) || !garden.contains(neighbour) {
                continue;
            }

            if let Some(next) = match garden[neighbour] {
                c if c != '#' => Some(neighbour),
                _ => None,
            } {
                visisted.insert(next);
                queue.push_back((next, steps - 1));
            }
        }
    }

    sum.len()
}

#[cfg(test)]
mod test {
    use utils::point::Point;

    use crate::year2023::day21::{explore, input};

    const EXAMPLE: &str = r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn input_test() {
        let (gardens, start) = input(EXAMPLE);
        assert_eq!(11, gardens.width);
        assert_eq!(11, gardens.height);
        assert_eq!(Point::new(5, 5), start);
    }

    #[test]
    fn part_one_test() {
        let (gardens, start) = input(EXAMPLE);
        assert_eq!(16, explore(&gardens, &start, 6));
    }
}
