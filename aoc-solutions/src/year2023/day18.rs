//! # Day 18:

use std::ops::Mul;

use utils::point::{Point, DOWN, LEFT, ORIGIN, RIGHT, UP};

type SolutionType = i64;

fn as_point(c: char, n: usize) -> Point {
    match c {
        'R' | '0' => RIGHT,
        'D' | '1' => DOWN,
        'L' | '2' => LEFT,
        'U' | '3' => UP,
        _ => unreachable!(),
    }
    .mul(n)
}

pub fn input(raw: &str) -> (Vec<Point>, Vec<Point>) {
    raw.lines()
        .map(|line| {
            let [direction, count, color] = line.split_ascii_whitespace().collect::<Vec<_>>()[..3]
            else {
                panic!("Failed to destructure line {}", line);
            };

            let simple_direction = direction.chars().nth(0).unwrap();
            let simple_count = count.parse::<usize>().unwrap();
            let simple_point = as_point(simple_direction, simple_count);

            let hash_value = color.chars().skip(2).take(5).collect::<String>();
            let hash_count = usize::from_str_radix(&hash_value, 16).unwrap();
            let hash_direction = color.chars().nth_back(1).unwrap();
            let hash_point = as_point(hash_direction, hash_count);

            (simple_point, hash_point)
        })
        .unzip()
}

pub fn part_one(input: &(Vec<Point>, Vec<Point>)) -> SolutionType {
    area(&input.0)
}

pub fn part_two(input: &(Vec<Point>, Vec<Point>)) -> SolutionType {
    area(&input.1)
}

fn area(points: &Vec<Point>) -> i64 {
    let mut points = points.iter();

    let mut corner = ORIGIN;
    let mut area = 0;
    let mut border = 0;

    while let Some(&next) = points.next() {
        area += corner.determinant(corner + next) as i64;
        border += corner.manhattan(corner + next) as i64;
        corner += next;
    }

    let internal_area = area.abs() / 2 - border / 2 + 1;

    border + internal_area
}

#[cfg(test)]
mod test {
    use utils::point::{DOWN, LEFT, RIGHT};

    use crate::year2023::day18::{input, part_one, part_two};

    const EXAMPLE: &str = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn input_test() {
        let (simple, hashed) = input(EXAMPLE);

        assert_eq!(vec![RIGHT * 6, DOWN * 5, LEFT * 2], simple[..3]);
        assert_eq!(
            vec![RIGHT * 461937, DOWN * 56407, RIGHT * 356671],
            hashed[..3]
        );
    }

    #[test]
    fn part_one_test() {
        assert_eq!(62, part_one(&input(EXAMPLE)));
    }

    #[test]
    fn part_two_test() {
        assert_eq!(952408144115, part_two(&input(EXAMPLE)));
    }
}
