//! # Day 22: Sand Slabs

use core::fmt;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    usize,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Range(usize, usize);

impl Range {
    fn intersects(self, other: Self) -> bool {
        // see https://stackoverflow.com/a/3269471
        self.0 <= other.1 && other.0 <= self.1
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.0, self.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Brick {
    x: Range,
    y: Range,
    z: Range,
}

#[derive(Debug)]
pub struct Supports {
    above: HashMap<Brick, HashSet<Brick>>,
    below: HashMap<Brick, HashSet<Brick>>,
}

impl Brick {
    fn parse(line: &str) -> Self {
        let coords = line
            .split([',', '~'])
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Brick {
            x: Range(coords[0], coords[3]),
            y: Range(coords[1], coords[4]),
            z: Range(coords[2], coords[5]),
        }
    }

    fn top(self) -> usize {
        self.z.1
    }

    fn bottom(self) -> usize {
        self.z.0
    }

    fn height(self) -> usize {
        self.top() - self.bottom()
    }

    fn intersects_xy(self, other: &Self) -> bool {
        self.x.intersects(other.x) && self.y.intersects(other.y)
    }
}

impl fmt::Display for Brick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub fn input(raw: &str) -> Vec<Brick> {
    raw.lines().map(Brick::parse).collect::<Vec<_>>()
}

pub fn part_one(input: &Vec<Brick>) -> usize {
    disintegrate(input).iter().filter(|&&x| x == 0).count()
}

pub fn part_two(input: &Vec<Brick>) -> usize {
    disintegrate(input).iter().sum()
}

fn disintegrate(bricks: &Vec<Brick>) -> Vec<usize> {
    let bricks = apply_gravity(&mut bricks.clone());
    let supports = supports(&bricks);

    let mut counts = vec![];

    for disintegrated in bricks {
        let mut queue = VecDeque::new();
        queue.push_back(disintegrated);

        let mut falling = HashSet::new();
        while let Some(brick) = queue.pop_front() {
            falling.insert(brick);

            if let Some(above) = supports.above.get(&brick) {
                // check if all of the supports for the block above are falling - if so, enqueue it
                above.iter().for_each(|b| {
                    supports.below[b]
                        .iter()
                        .all(|below| falling.contains(below))
                        .then(|| queue.push_back(*b));
                });
            }
        }
        counts.push(falling.len() - 1)
    }
    counts
}

fn apply_gravity(bricks: &mut Vec<Brick>) -> Vec<Brick> {
    let mut new_bricks: Vec<Brick> = vec![];

    // don't assume the input is sorted... wasted about 2 hours on this
    bricks.sort_by(|a, b| a.top().cmp(&b.top()));

    for (index, brick) in bricks.iter_mut().enumerate() {
        let mut bottom = 1;

        for previous in 0..index {
            let previous = new_bricks[previous];
            if previous.intersects_xy(&brick) {
                bottom = bottom.max(previous.top() + 1);
            }
        }
        let brick_height = brick.height();
        brick.z = Range(bottom, bottom + brick_height);
        new_bricks.push(*brick)
    }
    new_bricks
}

fn supports(bricks: &Vec<Brick>) -> Supports {
    let mut above: HashMap<Brick, HashSet<Brick>> = HashMap::new();
    let mut below: HashMap<Brick, HashSet<Brick>> = HashMap::new();

    for (index, current) in bricks.iter().enumerate() {
        for previous in bricks.iter().take(index) {
            let z_neighbours = previous.top() + 1 == current.bottom();
            if z_neighbours && previous.intersects_xy(current) {
                above
                    .entry(*previous)
                    .or_insert(HashSet::new())
                    .insert(*current);

                below
                    .entry(*current)
                    .or_insert(HashSet::new())
                    .insert(*previous);
            }
        }
    }
    Supports { above, below }
}

#[cfg(test)]
mod test {
    use crate::year2023::day22::{input, part_one, part_two, Brick, Range};

    const EXAMPLE: &str = r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn input_test() {
        let bricks = input(EXAMPLE);
        assert_eq!(
            Brick {
                x: Range(1, 1),
                y: Range(0, 2),
                z: Range(1, 1),
            },
            bricks[0]
        );
        assert_eq!(7, bricks.len())
    }

    #[test]
    fn part_one_test() {
        assert_eq!(5, part_one(&input(EXAMPLE)));
    }

    #[test]
    fn part_two_test() {
        assert_eq!(0, part_two(&input(EXAMPLE)));
    }
}
